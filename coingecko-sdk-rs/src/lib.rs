use std::collections::HashMap;

use indicatif::ProgressBar;
use log::error;
use serde::{Deserialize, Serialize};

use crate::client::dto::{CategoryMarketData, CoinMarketData};
use crate::client::CoinGeckoClient;
use crate::constants::{
    CATEGORY_SEPARATOR, MAX_MARKET_CAP, MIN_MARKET_CAP, TOP_COINS_FOR_FINAL_LIST,
    TOP_COINS_IN_CATEGORY, TOP_COINS_TOTAL,
};
use crate::io::{
    read_categories_data_from_file, write_categories_data_to_file, write_coin_info_to_file,
};

pub mod client;
mod constants;
mod io;

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
    coins: Vec<CoinMarketData>,
}

pub async fn output_top_coin_per_category(client: CoinGeckoClient) {
    let categories_data = match read_categories_data_from_file() {
        Ok(data) => data,
        Err(e) => {
            error!(
                "Error reading from file: {}. Fetching categories from API.",
                e
            );
            fetch_and_filter_categories_data(&client).await
        }
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
    coins.sort_by(|a, b| b.num_categories.cmp(&a.num_categories));
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
