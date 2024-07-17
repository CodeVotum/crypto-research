use std::{error, fs, io};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use csv::Writer;
use time::OffsetDateTime;

use crate::io::constants::{
    CACHE_FILE_PATH, CHANGE_SUMMARY_FILE_PREFIX, COIN_INFO_FILE_PREFIX,
    COIN_INFO_FILE_SUFFIX_FORMAT, OUTPUT_DIR_PATH,
};
use crate::models::{CategoryCoins, CoinInfo};
use crate::services::coin_diff::ChangeSummary;

mod constants;

pub fn read_categories_data_from_file() -> Result<HashMap<String, CategoryCoins>, Box<dyn Error>> {
    if Path::new(CACHE_FILE_PATH).exists() {
        let file = File::open(CACHE_FILE_PATH)?;
        let data = serde_json::from_reader(file)?;
        Ok(data)
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        )))
    }
}

pub fn read_coin_info_from_file(file_path: &str) -> Result<Vec<CoinInfo>, Box<dyn error::Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut coins = Vec::new();
    for result in rdr.deserialize() {
        let coin: CoinInfo = result?;
        coins.push(coin);
    }
    Ok(coins)
}

pub fn write_categories_data_to_file(categories: &HashMap<String, CategoryCoins>) {
    let file = File::create(CACHE_FILE_PATH).expect("Unable to create file");
    serde_json::to_writer_pretty(file, categories).expect("Unable to serialize categories");
}

pub fn write_coin_info_to_file(coin_info: &Vec<CoinInfo>) {
    let formatted_time = OffsetDateTime::now_utc()
        .format(COIN_INFO_FILE_SUFFIX_FORMAT)
        .unwrap();
    let file_name = format!(
        "{}/{}{}.csv",
        OUTPUT_DIR_PATH, COIN_INFO_FILE_PREFIX, formatted_time
    );
    let mut writer = Writer::from_path(file_name).expect("Unable to create file");

    for coin in coin_info {
        writer.serialize(coin).expect("Unable to write to CSV");
    }

    writer.flush().expect("Unable to flush writer");
}

pub fn write_change_summary_to_file(change_summary: &ChangeSummary) {
    let formatted_time = OffsetDateTime::now_utc()
        .format(COIN_INFO_FILE_SUFFIX_FORMAT)
        .unwrap();
    let file_name = format!(
        "{}/{}{}.json",
        OUTPUT_DIR_PATH, CHANGE_SUMMARY_FILE_PREFIX, formatted_time
    );
    let file = File::create(file_name).expect("Unable to create file");
    serde_json::to_writer_pretty(file, change_summary).expect("Unable to serialize change summary");
}

pub fn get_latest_output() -> Result<Vec<CoinInfo>, Box<dyn Error>> {
    let mut entries: Vec<_> = fs::read_dir(OUTPUT_DIR_PATH)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(COIN_INFO_FILE_PREFIX)
        })
        .collect();
    entries.sort_by_key(|entry| entry.metadata().and_then(|meta| meta.modified()).ok());
    let last = entries.into_iter().last().unwrap().path();
    read_coin_info_from_file(last.to_str().unwrap())
}
