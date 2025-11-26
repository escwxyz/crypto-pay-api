use rust_decimal::Decimal;
use serde::Serialize;

use crate::{
    models::CryptoCurrencyCode,
    utils::{serialize_comma_separated_list, serialize_decimal_to_string},
};

#[derive(Debug, Serialize, Default)]
pub struct GetTransfersParams {
    /// Optional. Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) asset: Option<CryptoCurrencyCode>,

    /// Optional. List of transfer IDs separated by comma.
    #[serde(
        serialize_with = "serialize_comma_separated_list",
        skip_serializing_if = "GetTransfersParams::should_skip_transfer_ids"
    )]
    pub(crate) transfer_ids: Option<Vec<u64>>,

    /// Optional. Unique UTF-8 transfer string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) spend_id: Option<String>,

    /// Optional. Offset needed to return a specific subset of transfers.
    /// Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<u32>,

    /// Optional. Number of transfers to be returned.
    /// Values between 1-1000 are accepted.
    /// Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<u16>,
}

impl GetTransfersParams {
    fn should_skip_transfer_ids(ids: &Option<Vec<u64>>) -> bool {
        !matches!(ids, Some(ids) if !ids.is_empty())
    }
}

#[derive(Debug, Serialize)]
pub struct TransferParams {
    /// User ID in Telegram. User must have previously used @CryptoBot (@CryptoTestnetBot for testnet).
    pub(crate) user_id: u64,

    /// Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub(crate) asset: CryptoCurrencyCode,

    /// Amount of the transfer in float.
    /// The minimum and maximum amount limits for each of the supported assets roughly correspond to 1-25000 USD.
    /// Use getExchangeRates to convert amounts. For example: 125.50
    #[serde(serialize_with = "serialize_decimal_to_string")]
    pub(crate) amount: Decimal,

    /// Random UTF-8 string unique per transfer for idempotent requests.
    /// The same spend_id can be accepted only once from your app.
    /// Up to 64 symbols.
    pub(crate) spend_id: String,

    /// Optional. Comment for the transfer.
    /// Users will see this comment in the notification about the transfer.
    /// Up to 1024 symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) comment: Option<String>,

    /// Optional. Pass true to not send to the user the notification about the transfer.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_send_notification: Option<bool>,
}
