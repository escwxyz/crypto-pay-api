use rust_decimal::Decimal;
use serde::Serialize;

use crate::{
    models::CryptoCurrencyCode,
    utils::{serialize_comma_separated_list, serialize_decimal_to_string},
};

use super::CheckStatus;

#[derive(Debug, Serialize)]
pub struct CreateCheckParams {
    /// Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub(crate) asset: CryptoCurrencyCode,

    /// Amount of the check in float. For example: 125.50
    #[serde(serialize_with = "serialize_decimal_to_string")]
    pub(crate) amount: Decimal,

    /// Optional. ID of the user who will be able to activate the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pin_to_user_id: Option<u64>,

    /// Optional. A user with the specified username will be able to activate the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pin_to_username: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct GetChecksParams {
    /// Optional. Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) asset: Option<CryptoCurrencyCode>,

    /// Optional. List of check IDs separated by comma.
    #[serde(
        serialize_with = "serialize_comma_separated_list",
        skip_serializing_if = "GetChecksParams::should_skip_check_ids"
    )]
    pub(crate) check_ids: Option<Vec<u64>>,

    /// Optional. Status of check to be returned. Available statuses: “active” and “activated”.
    /// Defaults to all statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<CheckStatus>,

    /// Optional. Offset needed to return a specific subset of check.
    /// Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<u32>,

    /// Optional. Number of check to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<u16>,
}

impl GetChecksParams {
    fn should_skip_check_ids(check_ids: &Option<Vec<u64>>) -> bool {
        !matches!(check_ids, Some(check_ids) if !check_ids.is_empty())
    }
}

#[derive(Debug, Serialize)]
pub struct DeleteCheckParams {
    pub check_id: u64,
}

impl From<u64> for DeleteCheckParams {
    fn from(check_id: u64) -> Self {
        Self { check_id }
    }
}
