use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BfMarketStatus {
    #[serde(rename = "PENDING_TRADING")]
    PendingTrading,
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "PRE_DELIVERING")]
    PreDelivering,
    #[serde(rename = "DELIVERING")]
    Delivering,
    #[serde(rename = "DELIVERED")]
    Delivered,
    #[serde(rename = "PRE_SETTLE")]
    PreSettle,
    #[serde(rename = "SETTLING")]
    Settling,
    #[serde(rename = "CLOSE")]
    Close,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BfMarketData {
    pub symbol: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    pub status: BfMarketStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BfExchangeInfo {
    #[serde(rename = "symbols")]
    pub markets: Vec<BfMarketData>,
}
