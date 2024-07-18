use std::collections::HashMap;

use log::error;

use binance_sdk_rs::client::BinanceFuturesClient;
use binance_sdk_rs::client::dto::{BfMarketData, BfMarketStatus};
use coingecko_sdk_rs::client::CoinGeckoClient;

use crate::models::{CategoryCoins, CoinInfo};
use crate::services::category::get_categories_data;
use crate::services::constants::{
    CATEGORY_SEPARATOR, MAX_MARKET_CAP, MIN_MARKET_CAP, TOP_COINS_FOR_FINAL_LIST,
};

pub async fn get_top_coins_per_category(
    cg_client: CoinGeckoClient,
    bf_client: BinanceFuturesClient,
) -> Vec<CoinInfo> {
    let binance_futures_markets = get_bf_markets(bf_client).await;
    let categories_data = get_categories_data(cg_client).await;
    collect_top_coins(binance_futures_markets, categories_data)
}

async fn get_bf_markets(bf_client: BinanceFuturesClient) -> Vec<String> {
    match bf_client.get_all_markets().await {
        Ok(markets) => prepare_bf_markets(markets),
        Err(e) => {
            error!("Error fetching Binance Futures markets: {}", e);
            Vec::new()
        }
    }
}

fn collect_top_coins(
    binance_futures_markets: Vec<String>,
    categories_sorted_by_market_cap: Vec<CategoryCoins>,
) -> Vec<CoinInfo> {
    let mut symbol_map = HashMap::new();

    for category_data in categories_sorted_by_market_cap {
        let category = &category_data.category;
        let coins = &category_data.coins;
        coins
            .iter()
            .filter(|coin| coin.market_cap_rank.is_some())
            .filter(|coin| binance_futures_markets.contains(&coin.symbol))
            .take(TOP_COINS_FOR_FINAL_LIST)
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
    symbol_map
        .values()
        .filter(|coin| matches!(coin.market_cap_rank, MIN_MARKET_CAP..=MAX_MARKET_CAP))
        .cloned()
        .collect()
}

fn prepare_bf_markets(markets: Vec<BfMarketData>) -> Vec<String> {
    markets
        .into_iter()
        .filter(|market| market.status == BfMarketStatus::Trading)
        .map(|market| market.base_asset.to_lowercase())
        .collect()
}
