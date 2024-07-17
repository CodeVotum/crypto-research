use serde::{Deserialize, Serialize};

use coingecko_sdk_rs::client::dto::{CategoryMarketData, CoinMarketData};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryCoins {
    pub category: CategoryMarketData,
    pub coins: Vec<CoinMarketData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinInfo {
    pub symbol: String,
    pub num_categories: usize,
    pub market_cap_rank: u16,
    pub categories: String,
}
