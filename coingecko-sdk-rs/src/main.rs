use env_logger::{init_from_env, Env};

use coingecko_sdk_rs::client::CoinGeckoClient;
use coingecko_sdk_rs::output_top_coin_per_category;

#[tokio::main]
async fn main() {
    init_from_env(Env::default().default_filter_or("info"));
    let client = CoinGeckoClient::default();
    output_top_coin_per_category(client).await;
}
