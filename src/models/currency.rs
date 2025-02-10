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
}

impl std::fmt::Display for CryptoCurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
}

impl std::fmt::Display for FiatCurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyType {
    Crypto,
    Fiat,
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
