use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use csv::Writer;
use env_logger::{init_from_env, Env};
use indicatif::ProgressBar;
use log::error;
use serde::{Deserialize, Serialize};

use coingecko_rs::constants::CACHE_FILE_PATH;
use coingecko_rs::{CategoryMarketData, CoinGeckoClient};

const TOP_COINS_TOTAL: u8 = 10;
const TOP_COINS_IN_CATEGORY: u8 = 10;
const TOP_COINS_FOR_FINAL_LIST: usize = 1;
const COIN_INFO_FILE_PATH: &str = "coin_info.csv";
const MIN_MARKET_CAP: u16 = 1;
const MAX_MARKET_CAP: u16 = 500;
const CATEGORY_SEPARATOR: &str = ", ";

#[derive(Serialize, Debug)]
struct CoinInfo {
    symbol: String,
    num_categories: usize,
    market_cap_rank: u16,
    categories: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryCoins {
    category: CategoryMarketData,
    coins: Vec<coingecko_rs::CoinMarketData>,
}

#[tokio::main]
async fn main() {
    init_from_env(Env::default().default_filter_or("info"));

    let client = CoinGeckoClient::default();

    let categories_data = if Path::new(CACHE_FILE_PATH).exists() {
        read_categories_data_from_file().unwrap_or_else(|e| {
            error!(
                "Error reading from file: {}. Fetching categories from API.",
                e
            );
            HashMap::new()
        })
    } else {
        fetch_and_filter_categories_data(&client).await
    };

    let mut symbol_map = HashMap::new();

    for category_data in categories_data.values() {
        let category = &category_data.category;
        let coins = &category_data.coins;
        coins
            .iter()
            .take(TOP_COINS_FOR_FINAL_LIST)
            .filter(|coin| coin.market_cap_rank.is_some())
            .for_each(|coin| {
                let entry = symbol_map.entry(coin.symbol.clone()).or_insert(CoinInfo {
                    symbol: coin.symbol.clone(),
                    num_categories: 0,
                    market_cap_rank: 0,
                    categories: String::new(),
                });
                entry.num_categories += 1;
                entry.market_cap_rank = coin.market_cap_rank.unwrap();
                if !entry.categories.is_empty() {
                    entry.categories.push_str(CATEGORY_SEPARATOR);
                }
                entry.categories.push_str(&category.name);
            });
    }

    let mut coins: Vec<&CoinInfo> = symbol_map
        .values()
        .filter(|coin| matches!(coin.market_cap_rank, MIN_MARKET_CAP..=MAX_MARKET_CAP))
        .collect();
    coins.sort_by(|a, b| b.num_categories.cmp(&a.num_categories).reverse());
    write_coin_info_to_file(&coins);
}

async fn fetch_and_filter_categories_data(
    client: &CoinGeckoClient,
) -> HashMap<String, CategoryCoins> {
    let categories = match client.get_categories_market_data().await {
        Ok(categories) => categories,
        Err(e) => {
            error!("Error fetching categories: {}", e);
            return HashMap::new();
        }
    };

    let top_coins_ids: Vec<String> = match client.get_coins_market_data(TOP_COINS_TOTAL).await {
        Ok(coins) => coins.iter().map(|coin| coin.id.clone()).collect(),
        Err(e) => {
            error!("Error fetching coins market data: {}", e);
            return HashMap::new();
        }
    };

    let mut filtered_categories = HashMap::new();

    let bar = ProgressBar::new(categories.len() as u64);
    for category in categories {
        bar.inc(1);
        match client
            .get_coins_in_category(&category.id, TOP_COINS_IN_CATEGORY)
            .await
        {
            Ok(coins) => {
                if coins.is_empty() || top_coins_ids.contains(&coins[0].id) {
                    continue;
                }
                filtered_categories.insert(category.id.clone(), CategoryCoins { category, coins });
            }
            Err(e) => error!("Error fetching coins in category {}: {}", category.name, e),
        }
    }
    bar.finish_with_message("Done!");

    write_categories_data_to_file(&filtered_categories);

    filtered_categories
}

fn read_categories_data_from_file() -> Result<HashMap<String, CategoryCoins>, Box<dyn Error>> {
    let file = File::open(CACHE_FILE_PATH).expect("Unable to open file");
    Ok(serde_json::from_reader(file)?)
}

fn write_categories_data_to_file(categories: &HashMap<String, CategoryCoins>) {
    let file = File::create(CACHE_FILE_PATH).expect("Unable to create file");
    serde_json::to_writer_pretty(file, categories).expect("Unable to serialize categories");
}

fn write_coin_info_to_file(coin_info: &Vec<&CoinInfo>) {
    let mut writer = Writer::from_path(COIN_INFO_FILE_PATH).expect("Unable to create file");

    for coin in coin_info {
        writer.serialize(coin).expect("Unable to write to CSV");
    }

    writer.flush().expect("Unable to flush writer");
}
