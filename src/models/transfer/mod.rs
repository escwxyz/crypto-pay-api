mod builder;
mod params;

pub use builder::*;
pub use params::*;

use super::CryptoCurrencyCode;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::utils::deserialize_decimal_from_string;

#[derive(Debug, Deserialize)]
pub struct Transfer {
    /// Unique ID for this transfer.
    pub transfer_id: u64,

    /// Unique UTF-8 string.
    pub spend_id: String,

    /// Telegram user ID the transfer was sent to.
    pub user_id: u64,

    /// Cryptocurrency alphabetic code. Currently, can be “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub asset: CryptoCurrencyCode,

    /// Amount of the transfer in float.
    #[serde(deserialize_with = "deserialize_decimal_from_string")]
    pub amount: Decimal,

    /// Status of the transfer, can only be “completed”.
    pub status: TransferStatus,

    /// Date the transfer was completed in ISO 8601 format.
    pub completed_at: DateTime<Utc>,

    /// Optional. Comment for this transfer.
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransferStatus {
    Completed,
}
