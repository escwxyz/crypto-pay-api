use std::fmt::Display;

use crate::utils::deserialize_currency_code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub is_blockchain: bool,
    pub is_stablecoin: bool,
    pub is_fiat: bool,
    pub name: String,
    #[serde(deserialize_with = "deserialize_currency_code")]
    pub code: CurrencyCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CurrencyCode {
    Crypto(CryptoCurrencyCode),
    Fiat(FiatCurrencyCode),
}

#[cfg(not(tarpaulin))]
impl Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyCode::Crypto(code) => write!(f, "{code}"),
            CurrencyCode::Fiat(code) => write!(f, "{code}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CryptoCurrencyCode {
    Usdt,
    Ton,
    Btc,
    Eth,
    Ltc,
    Bnb,
    Trx,
    Usdc,
    Doge,
    Send,
    Jet,
    #[serde(other)]
    Unknown,
}

#[cfg(not(tarpaulin))]
impl Display for CryptoCurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum FiatCurrencyCode {
    Usd,
    Eur,
    Rub,
    Byn,
    Uah,
    Gbp,
    Cny,
    Kgs,
    Kzt,
    Uzs,
    Gel,
    Try,
    Amd,
    Thb,
    Tjs,
    Inr,
    Brl,
    Idr,
    Azn,
    Aed,
    Pln,
    Ils,
    Lkr,
    #[serde(other)]
    Unknown,
}

#[cfg(not(tarpaulin))]
impl Display for FiatCurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyType {
    Crypto,
    Fiat,
}

#[cfg(not(tarpaulin))]
impl Display for CurrencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<CryptoCurrencyCode> for CurrencyCode {
    fn from(code: CryptoCurrencyCode) -> Self {
        CurrencyCode::Crypto(code)
    }
}

impl From<FiatCurrencyCode> for CurrencyCode {
    fn from(code: FiatCurrencyCode) -> Self {
        CurrencyCode::Fiat(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_type_serialization() {
        let crypto = CurrencyType::Crypto;
        let fiat = CurrencyType::Fiat;

        assert_eq!(serde_json::to_string(&crypto).unwrap(), "\"crypto\"");
        assert_eq!(serde_json::to_string(&fiat).unwrap(), "\"fiat\"");
    }

    #[test]
    fn test_crypto_currency_code_serialization() {
        let btc = CryptoCurrencyCode::Btc;
        let ton = CryptoCurrencyCode::Ton;

        assert_eq!(serde_json::to_string(&btc).unwrap(), "\"BTC\"");
        assert_eq!(serde_json::to_string(&ton).unwrap(), "\"TON\"");
    }

    #[test]
    fn test_fiat_currency_code_serialization() {
        let usd = FiatCurrencyCode::Usd;
        let eur = FiatCurrencyCode::Eur;

        assert_eq!(serde_json::to_string(&usd).unwrap(), "\"USD\"");
        assert_eq!(serde_json::to_string(&eur).unwrap(), "\"EUR\"");
    }

    #[test]
    fn test_currency_code_deserialization() {
        let crypto: CryptoCurrencyCode = serde_json::from_str("\"BTC\"").unwrap();
        let fiat: FiatCurrencyCode = serde_json::from_str("\"USD\"").unwrap();

        assert_eq!(crypto, CryptoCurrencyCode::Btc);
        assert_eq!(fiat, FiatCurrencyCode::Usd);

        assert_eq!(
            serde_json::from_str::<CryptoCurrencyCode>("\"btc\"").unwrap(),
            CryptoCurrencyCode::Unknown
        );
        assert_eq!(
            serde_json::from_str::<FiatCurrencyCode>("\"usd\"").unwrap(),
            FiatCurrencyCode::Unknown
        );
    }

    #[test]
    fn test_currency_code_conversion() {
        let crypto = CryptoCurrencyCode::Btc;
        let fiat = FiatCurrencyCode::Usd;

        let currency_code_crypto: CurrencyCode = crypto.into();
        let currency_code_fiat: CurrencyCode = fiat.into();

        assert!(matches!(
            currency_code_crypto,
            CurrencyCode::Crypto(CryptoCurrencyCode::Btc)
        ));
        assert!(matches!(currency_code_fiat, CurrencyCode::Fiat(FiatCurrencyCode::Usd)));
    }
}
