pub const COINGECKO_API_URL: &str = "https://api.coingecko.com/api/v3";
pub const USER_AGENT: &str = "coingecko_sdk_rs";
pub const TOKEN_HEADER: &str = "x-cg-demo-api-key";
pub const TOKEN_ENV_VAR: &str = "COINGECKO_TOKEN";
//TODO: this should depend on coingecko's plan. Need to test concurrency.
pub const SECONDS_TO_WAIT: u64 = 2;
pub const MAX_CONCURRENT_REQUESTS: usize = 30;
