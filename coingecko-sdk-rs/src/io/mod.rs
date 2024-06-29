use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::{error, io};

use csv::Writer;

use crate::io::constants::{CACHE_FILE_PATH, COIN_INFO_FILE_PATH};
use crate::{CategoryCoins, CoinInfo};

mod constants;

pub fn read_categories_data_from_file(
) -> Result<HashMap<String, CategoryCoins>, Box<dyn error::Error>> {
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

pub fn write_categories_data_to_file(categories: &HashMap<String, CategoryCoins>) {
    let file = File::create(CACHE_FILE_PATH).expect("Unable to create file");
    serde_json::to_writer_pretty(file, categories).expect("Unable to serialize categories");
}

pub fn write_coin_info_to_file(coin_info: &Vec<&CoinInfo>) {
    let mut writer = Writer::from_path(COIN_INFO_FILE_PATH).expect("Unable to create file");

    for coin in coin_info {
        writer.serialize(coin).expect("Unable to write to CSV");
    }

    writer.flush().expect("Unable to flush writer");
}
