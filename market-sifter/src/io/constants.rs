use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

pub const OUTPUT_DIR_PATH: &str = "output";
pub const CACHE_FILE_PATH: &str = "output/filtered_categories.json";
pub const COIN_INFO_FILE_PREFIX: &str = "coin_info_";
pub const CHANGE_SUMMARY_FILE_PREFIX: &str = "change_summary_";
pub const COIN_INFO_FILE_SUFFIX_FORMAT: &[BorrowedFormatItem] =
    format_description!("[year]-[month]-[day]_[hour]-[minute]");
