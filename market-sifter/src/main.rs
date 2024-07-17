use coingecko_sdk_rs::client::CoinGeckoClient;

use crate::io::{get_latest_output, write_change_summary_to_file, write_coin_info_to_file};
use crate::services::coin_diff::compare_coin_lists;
use crate::services::top_coins::get_top_coins_per_category;

mod io;
mod models;
mod services;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = CoinGeckoClient::default();

    let previous_output = get_latest_output().unwrap();

    let top_coins_per_category = get_top_coins_per_category(client).await;

    write_coin_info_to_file(&top_coins_per_category);

    let change_summary = compare_coin_lists(previous_output, top_coins_per_category);

    write_change_summary_to_file(&change_summary);
}
