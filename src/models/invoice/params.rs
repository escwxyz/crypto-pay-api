use rust_decimal::Decimal;
use serde::Serialize;

use crate::{
    models::{CryptoCurrencyCode, CurrencyType, FiatCurrencyCode, PayButtonName},
    utils::{serialize_comma_separated_list, serialize_decimal_to_string},
};

use super::InvoiceStatus;

/* #region GetInvoicesParams */
#[derive(Debug, Default, Serialize)]
pub struct GetInvoicesParams {
    /// Optional. Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) asset: Option<CryptoCurrencyCode>,

    /// Optional. Fiat currency code. Supported fiat currencies: “USD”, “EUR”, “RUB”, “BYN”, “UAH”, “GBP”, “CNY”, “KZT”, “UZS”, “GEL”, “TRY”, “AMD”, “THB”, “INR”, “BRL”, “IDR”, “AZN”, “AED”, “PLN” and “ILS".
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fiat: Option<FiatCurrencyCode>,

    /// Optional. List of invoice IDs separated by comma.
    #[serde(
        serialize_with = "serialize_comma_separated_list",
        skip_serializing_if = "GetInvoicesParams::should_skip_invoice_ids"
    )]
    pub(crate) invoice_ids: Option<Vec<u64>>,

    /// Optional. Status of invoices to be returned. Available statuses: “active” and “paid”.
    /// Defaults to all statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<InvoiceStatus>,

    /// Optional. Offset needed to return a specific subset of invoices.
    /// Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<u32>,

    /// Optional. Number of invoices to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<u16>,
}

impl GetInvoicesParams {
    fn should_skip_invoice_ids(ids: &Option<Vec<u64>>) -> bool {
        !matches!(ids, Some(ids) if !ids.is_empty())
    }
}

/* #endregion */

/* #region CreateInvoiceParams */

#[derive(Debug, Serialize)]
pub struct CreateInvoiceParams {
    /// Optional. Type of the price, can be "crypto" or "fiat". Defaults to crypto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) currency_type: Option<CurrencyType>,

    /// Optional.  Required if currency_type is "crypto". Cryptocurrency alphabetic code. Supported assets: "USDT", "TON", "BTC", "ETH", "LTC", "BNB", "TRX" and "USDC".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) asset: Option<CryptoCurrencyCode>,

    /// Optional. Required if currency_type is "fiat". Fiat currency code. Supported fiat currencies: "USD", "EUR", "RUB", "BYN", "UAH", "GBP", "CNY", "KZT", "UZS", "GEL", "TRY", "AMD", "THB", "INR", "BRL", "IDR", "AZN", "AED", "PLN" and "ILS".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fiat: Option<FiatCurrencyCode>,

    /// Optional. List of cryptocurrency alphabetic codes separated comma. Assets which can be used to pay the invoice. Available only if currency_type is "crypto". Supported assets: "USDT", "TON", "BTC", "ETH", "LTC", "BNB", "TRX" and "USDC" ("JET" for testnet). Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) accept_asset: Option<Vec<CryptoCurrencyCode>>,

    /// Amount of the invoice in float. For example: 125.50
    #[serde(serialize_with = "serialize_decimal_to_string")]
    pub(crate) amount: Decimal,

    /// Optional. Description for the invoice. User will see this description when they pay the invoice. Up to 1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,

    /// Optional. Text of the message which will be presented to a user after the invoice is paid. Up to 2048 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hidden_message: Option<String>,

    /// Optional. Label of the button which will be presented to a user after the invoice is paid.
    /// Supported names:
    /// viewItem – "View Item",
    /// openChannel – "View Channel",
    /// openBot – "Open Bot",
    /// callback – "Return to the bot"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) paid_btn_name: Option<PayButtonName>,

    /// Optional. Required if paid_btn_name is specified. URL opened using the button which will be presented to a user after the invoice is paid.
    /// You can set any callback link (for example, a success link or link to homepage).
    /// Starts with https or http.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) paid_btn_url: Option<String>,

    /// Optional. Any data you want to attach to the invoice (for example, user ID, payment ID, ect).
    /// Up to 4kb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) payload: Option<String>,

    /// Optional. Allow a user to add a comment to the payment.
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allow_comments: Option<bool>,

    /// Optional. Allow a user to pay the invoice anonymously.
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allow_anonymous: Option<bool>,

    /// Optional. You can set a payment time limit for the invoice in seconds.
    /// Values between 1-2678400 are accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expires_in: Option<u32>,
}

/* #endregion */

/* #region DeleteInvoiceParams */

#[derive(Debug, Serialize)]
pub struct DeleteInvoiceParams {
    pub(crate) invoice_id: u64,
}
