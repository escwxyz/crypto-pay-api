mod params;

use chrono::{DateTime, Utc};
pub use params::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::utils::deserialize_decimal;

use super::CryptoCurrencyCode;

#[derive(Debug, Deserialize)]
pub struct Check {
    /// Unique ID for this check.
    pub check_id: u64,

    /// Hash of the check.
    pub hash: String,

    /// Cryptocurrency alphabetic code. Currently, can be “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub asset: CryptoCurrencyCode,

    /// Amount of the check in float.
    #[serde(deserialize_with = "deserialize_decimal")]
    pub amount: Decimal,

    /// URL should be provided to the user to activate the check.
    pub bot_check_url: String,

    /// Status of the check, can be “active” or “activated”.
    pub status: CheckStatus,

    /// Date the check was created in ISO 8601 format.
    pub created_at: DateTime<Utc>,

    /// Date the check was activated in ISO 8601 format.
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Active,
    Activated,
}
