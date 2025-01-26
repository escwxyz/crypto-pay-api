use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub is_blockchain: bool,
    pub is_stablecoin: bool,
    pub is_fiat: bool,
    pub name: String,
    pub code: CryptoCurrencyCode,
    pub url: String,
    pub decimals: u8,
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
    Jet, // Only for testnet
}

impl std::fmt::Display for CryptoCurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
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
    Kzt,
    Uzs,
    Gel,
    Try,
    Amd,
    Thb,
    Inr,
    Brl,
    Idr,
    Azn,
    Aed,
    Pln,
    Ils,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error: Option<String>,
    pub error_code: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GetMeResponse {
    pub app_id: i64,
    pub name: String,
    pub payment_processing_bot_username: String,
    pub webhook_endpoint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CurrencyType {
    Crypto,
    Fiat,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PayButtonName {
    ViewItem,
    OpenChannel,
    OpenBot,
    Callback,
}
