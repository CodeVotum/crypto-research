use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinMarketData {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryMarketData {
    pub id: String,
    pub name: String,
    pub market_cap: Option<f64>,
}
