use super::CryptoCurrencyCode;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Balance {
    /// Cryptocurrency alphabetic code.
    /// Currently, can be “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX”, "SEND" and “USDC” (and “JET” for testnet).
    pub currency_code: CryptoCurrencyCode,

    /// Total available amount in float.
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_decimal_from_string")]
    pub available: Decimal,

    /// Unavailable amount currently is on hold in float.
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_decimal_from_string")]
    pub onhold: Decimal,
}
