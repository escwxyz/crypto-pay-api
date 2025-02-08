use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Official API methods
#[derive(Debug, Serialize, Deserialize)]
pub enum ApiMethod {
    #[serde(rename = "getMe")]
    GetMe,
    #[serde(rename = "createInvoice")]
    CreateInvoice,
    #[serde(rename = "deleteInvoice")]
    DeleteInvoice,
    #[serde(rename = "createCheck")]
    CreateCheck,
    #[serde(rename = "deleteCheck")]
    DeleteCheck,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "getInvoices")]
    GetInvoices,
    #[serde(rename = "getChecks")]
    GetChecks,
    #[serde(rename = "getTransfers")]
    GetTransfers,
    #[serde(rename = "getBalance")]
    GetBalance,
    #[serde(rename = "getExchangeRates")]
    GetExchangeRates,
    #[serde(rename = "getCurrencies")]
    GetCurrencies,
    #[serde(rename = "getStats")]
    GetStats,
}

impl std::fmt::Display for ApiMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct AppStats {
    /// Total volume of paid invoices in USD.
    pub volume: u64,
    /// Conversion of all created invoices.
    pub conversion: u64,
    /// The unique number of users who have paid the invoice.
    pub unique_user_count: u64,
    /// Total created invoice count.
    pub created_invoice_count: u64,
    /// Total paid invoice count.
    pub paid_invoice_count: u64,
    /// The date on which the statistics calculation was started in ISO 8601 format.
    pub start_at: DateTime<Utc>,
    /// The date on which the statistics calculation was ended in ISO 8601 format.
    pub end_at: DateTime<Utc>,
}

// TODO: Add validation
#[derive(Debug, Deserialize, Serialize)]
pub struct GetStatsParams {
    /// Optional. Date from which start calculating statistics in ISO 8601 format. Defaults is current date minus 24 hours.
    pub start_at: Option<DateTime<Utc>>,
    /// Optional. Date to which end calculating statistics in ISO 8601 format. Defaults is current date.
    pub end_at: Option<DateTime<Utc>>,
}
// TODO
impl std::default::Default for GetStatsParams {
    fn default() -> Self {
        Self {
            start_at: Some(Utc::now() - chrono::Duration::hours(24)),
            end_at: Some(Utc::now()),
        }
    }
}
