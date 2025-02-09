use crate::utils::{deserialize_decimal_from_string, serialize_decimal_to_string};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{CryptoCurrencyCode, FiatCurrencyCode};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// True, if the received rate is up-to-date.
    pub is_valid: bool,
    /// True, if the source currency is a cryptocurrency.
    pub is_crypto: bool,
    /// True, if the source is the fiat currency.
    pub is_fiat: bool,
    /// Cryptocurrency alphabetic code. Currently, can be “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC”.
    pub source: CryptoCurrencyCode,
    /// Fiat currency code. Currently, can be “USD”, “EUR”, “RUB”, “BYN”, “UAH”, “GBP”, “CNY”, “KZT”, “UZS”, “GEL”, “TRY”, “AMD”, “THB”, “INR”, “BRL”, “IDR”, “AZN”, “AED”, “PLN” and “ILS".
    pub target: FiatCurrencyCode,
    /// The current rate of the source asset valued in the target currency.
    #[serde(deserialize_with = "deserialize_decimal_from_string")]
    #[serde(serialize_with = "serialize_decimal_to_string")]
    pub rate: Decimal, // 1 source = rate target
}
