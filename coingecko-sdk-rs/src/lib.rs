use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use log::debug;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::sync::Semaphore;
use tokio::time::sleep;

use crate::constants::{
    COINGECKO_API_URL, MAX_CONCURRENT_REQUESTS, SECONDS_TO_WAIT, TOKEN_ENV_VAR, TOKEN_HEADER,
    USER_AGENT,
};

pub mod constants;

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

pub struct CoinGeckoClient {
    client: Client,
    base_url: String,
    rate_limiter: Arc<Semaphore>,
}

impl Default for CoinGeckoClient {
    fn default() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );
        let token = std::env::var(TOKEN_ENV_VAR)
            .unwrap_or_else(|_| panic!("{} must be set", TOKEN_ENV_VAR));
        headers.insert(
            header::HeaderName::from_static(TOKEN_HEADER),
            header::HeaderValue::from_str(&token).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        let rate_limiter = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));
        CoinGeckoClient {
            client,
            base_url: COINGECKO_API_URL.to_string(),
            rate_limiter,
        }
    }
}

impl CoinGeckoClient {
    async fn rate_limited_request<F, T>(&self, request_fn: F) -> Result<T, Box<dyn Error>>
    where
        F: Fn() -> reqwest::RequestBuilder,
        T: serde::de::DeserializeOwned,
    {
        let _permit = self.rate_limiter.acquire().await.unwrap();
        sleep(Duration::from_secs(SECONDS_TO_WAIT)).await;
        debug!(
            "Making request, current ts: {}, remaining permits: {}",
            OffsetDateTime::now_utc(),
            self.rate_limiter.available_permits()
        );
        let response = request_fn().send().await?;
        Ok(response.json().await?)
    }

    pub async fn get_coins_market_data(
        &self,
        limit: u8,
    ) -> Result<Vec<CoinMarketData>, Box<dyn Error>> {
        let url = format!(
            "{}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}",
            self.base_url, limit
        );
        self.rate_limited_request(|| self.client.get(&url)).await
    }

    pub async fn get_categories_market_data(
        &self,
    ) -> Result<Vec<CategoryMarketData>, Box<dyn Error>> {
        let url = format!("{}/coins/categories?order=market_cap_desc", self.base_url);
        self.rate_limited_request(|| self.client.get(&url)).await
    }

    pub async fn get_coins_in_category(
        &self,
        category_id: &str,
        limit: u8,
    ) -> Result<Vec<CoinMarketData>, Box<dyn Error>> {
        let url = format!(
            "{}/coins/markets?vs_currency=usd&category={}&order=market_cap_desc&per_page={}",
            self.base_url, category_id, limit
        );
        self.rate_limited_request(|| self.client.get(&url)).await
    }
}
