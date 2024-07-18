use std::error::Error;

use reqwest::Client;

use crate::client::constants::*;
use crate::client::dto::{BfExchangeInfo, BfMarketData};

mod constants;
pub mod dto;

pub struct BinanceFuturesClient {
    client: Client,
    base_url: String,
}

impl Default for BinanceFuturesClient {
    fn default() -> Self {
        BinanceFuturesClient {
            client: Client::new(),
            base_url: API_URL.to_string(),
        }
    }
}

impl BinanceFuturesClient {
    pub async fn get_all_markets(&self) -> Result<Vec<BfMarketData>, Box<dyn Error>> {
        Ok(self.get_exchange_info().await?.markets)
    }

    async fn get_exchange_info(&self) -> Result<BfExchangeInfo, Box<dyn Error>> {
        let url = format!("{}/exchangeInfo", self.base_url);
        Ok(self.client.get(&url).send().await?.json().await?)
    }
}
