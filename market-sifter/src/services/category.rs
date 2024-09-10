use std::collections::HashMap;

use indicatif::ProgressBar;
use log::{error, warn};

use coingecko_sdk_rs::client::CoinGeckoClient;

use crate::io::{read_categories_data_from_file, write_categories_data_to_file};
use crate::models::CategoryCoins;
use crate::services::constants::{TOP_COINS_IN_CATEGORY, TOP_COINS_TOTAL};

pub async fn get_categories_data(cg_client: CoinGeckoClient) -> Vec<CategoryCoins> {
    let categories_data = match read_categories_data_from_file() {
        Ok(data) => data,
        Err(e) => {
            warn!(
                "Error reading from file: {}. Fetching categories from API.",
                e
            );
            fetch_categories_data(cg_client).await
        }
    };
    let mut categories_sorted_by_market_cap: Vec<CategoryCoins> =
        categories_data.values().cloned().collect();
    categories_sorted_by_market_cap.sort_by(|a, b| a.category.id.cmp(&b.category.id));
    categories_sorted_by_market_cap
}

async fn fetch_categories_data(cg_client: CoinGeckoClient) -> HashMap<String, CategoryCoins> {
    let categories_data = fetch_and_filter_categories_data(cg_client).await;
    write_categories_data_to_file(&categories_data);
    categories_data
}

async fn fetch_and_filter_categories_data(
    client: CoinGeckoClient,
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
    filtered_categories
}
