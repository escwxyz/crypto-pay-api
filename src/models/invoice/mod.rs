mod builder;
mod params;

pub use builder::*;
use chrono::{DateTime, Utc};
pub use params::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{CryptoCurrencyCode, CurrencyType, FiatCurrencyCode, PayButtonName};
use crate::utils::{deserialize_decimal_from_string, deserialize_optional_decimal_from_string};

#[derive(Debug, Deserialize, Clone)]
pub struct Invoice {
    /// Unique ID for this invoice.
    pub invoice_id: i64,

    /// Hash of the invoice.
    pub hash: String,

    /// Type of the price, can be "crypto" or "fiat".
    pub currency_type: CurrencyType,

    /// Optional. Cryptocurrency code. Available only if the value of the field currency_type is "crypto". Currently, can be "USDT", "TON", "BTC", "ETH", "LTC", "BNB", "TRX" and "USDC" (and "JET" for testnet).
    pub asset: Option<CryptoCurrencyCode>,

    /// Optional. Fiat currency code. Available only if the value of the field currency_type is "fiat". Currently one of "USD", "EUR", "RUB", "BYN", "UAH", "GBP", "CNY", "KZT", "UZS", "GEL", "TRY", "AMD", "THB", "INR", "BRL", "IDR", "AZN", "AED", "PLN" and "ILS".
    pub fiat: Option<FiatCurrencyCode>,

    /// Amount of the invoice for which the invoice was created.
    #[serde(deserialize_with = "deserialize_decimal_from_string")]
    pub amount: Decimal,

    /// Optional. Cryptocurrency alphabetic code for which the invoice was paid. Available only if currency_type is "crypto" and status is "paid".
    pub paid_asset: Option<CryptoCurrencyCode>,

    /// Optional. Amount of the invoice for which the invoice was paid. Available only if currency_type is "fiat" and status is "paid".
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_decimal_from_string")]
    pub paid_amount: Option<Decimal>,

    /// Optional. The rate of the paid_asset valued in the fiat currency. Available only if the value of the field currency_type is "fiat" and the value of the field status is "paid".
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_decimal_from_string")]
    pub paid_fiat_rate: Option<Decimal>,

    /// Optional. List of assets which can be used to pay the invoice. Available only if currency_type is "fiat". Currently, can be "USDT", "TON", "BTC", "ETH", "LTC", "BNB", "TRX" and "USDC" ("JET" for testnet).
    pub accept_asset: Option<Vec<CryptoCurrencyCode>>,

    /// Optional. Asset of service fees charged when the invoice was paid. Available only if status is "paid".
    pub fee_asset: Option<String>,

    /// Optional. Amount of service fees charged when the invoice was paid. Available only if status is "paid".
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_decimal_from_string")]
    pub fee_amount: Option<Decimal>,

    /// URL should be provided to the user to pay the invoice.
    pub bot_invoice_url: String,

    /// Use this URL to pay an invoice to the Telegram Mini App version.
    pub mini_app_invoice_url: String,

    /// Use this URL to pay an invoice to the Web version of Crypto Bot.
    pub web_app_invoice_url: String,

    /// Optional. Description for this invoice.
    pub description: Option<String>,

    /// Status of the transfer, can be "active", "paid" or "expired".
    pub status: InvoiceStatus,

    /// Date the invoice was created in ISO 8601 format.
    pub created_at: DateTime<Utc>,

    /// Optional. Price of the asset in USD. Available only if status is "paid".
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_decimal_from_string")]
    pub paid_usd_rate: Option<Decimal>,

    /// True, if the user can add comment to the payment.
    pub allow_comments: bool,

    /// True, if the user can pay the invoice anonymously.
    pub allow_anonymous: bool,

    /// Optional. Date the invoice expires in ISO 8601 format.
    pub expires_date: Option<DateTime<Utc>>,

    /// Optional. Date the invoice was paid in ISO 8601 format.
    pub paid_at: Option<DateTime<Utc>>,

    /// True, if the invoice was paid anonymously.
    pub paid_anonymously: Option<bool>,

    /// Optional. Comment to the payment from the user.
    pub comment: Option<String>,

    /// Optional. Text of the hidden message for this invoice.
    pub hidden_message: Option<String>,

    /// Optional. Previously provided data for this invoice.
    pub payload: Option<String>,

    /// Optional. Label of the button, can be "viewItem", "openChannel", "openBot" or "callback".
    pub paid_btn_name: Option<PayButtonName>,

    /// Optional. URL opened using the button.
    pub paid_btn_url: Option<String>,
}

// Customized methods to put here in the struct
impl Invoice {
    pub fn is_paid(&self) -> bool {
        self.status == InvoiceStatus::Paid
    }

    pub fn is_expired(&self) -> bool {
        self.status == InvoiceStatus::Expired
    }

    // TODO
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceStatus {
    Active,
    Paid,
    Expired,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvoiceStatus::Active => write!(f, "active"),
            InvoiceStatus::Paid => write!(f, "paid"),
            InvoiceStatus::Expired => write!(f, "expired"),
        }
    }
}
