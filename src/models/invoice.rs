use crate::{
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    utils::{
        deserialize_decimal_from_string, deserialize_optional_decimal_from_string,
        serialize_comma_separated_list, serialize_decimal_to_string,
    },
    validate_dependency,
    validation::{
        validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext,
    },
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{CryptoCurrencyCode, CurrencyType, FiatCurrencyCode, PayButtonName};

// ---- Invoice ----

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

// ---- GetInvoicesParams ----

#[derive(Debug, Default, Serialize)]
pub struct GetInvoicesParams {
    /// Optional. Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<CryptoCurrencyCode>,

    /// Optional. Fiat currency code. Supported fiat currencies: “USD”, “EUR”, “RUB”, “BYN”, “UAH”, “GBP”, “CNY”, “KZT”, “UZS”, “GEL”, “TRY”, “AMD”, “THB”, “INR”, “BRL”, “IDR”, “AZN”, “AED”, “PLN” and “ILS".
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat: Option<FiatCurrencyCode>,

    /// Optional. List of invoice IDs separated by comma.
    #[serde(
        serialize_with = "serialize_comma_separated_list",
        skip_serializing_if = "GetInvoicesParams::should_skip_invoice_ids"
    )]
    pub invoice_ids: Option<Vec<u64>>,

    /// Optional. Status of invoices to be returned. Available statuses: “active” and “paid”.
    /// Defaults to all statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,

    /// Optional. Offset needed to return a specific subset of invoices.
    /// Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Optional. Number of invoices to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
}

impl GetInvoicesParams {
    fn should_skip_invoice_ids(ids: &Option<Vec<u64>>) -> bool {
        !matches!(ids, Some(ids) if !ids.is_empty())
    }
}

impl FieldValidate for GetInvoicesParams {
    fn validate(&self) -> CryptoBotResult<()> {
        if let Some(count) = &self.count {
            validate_count(*count)?;
        }
        Ok(())
    }
}

// ---- CreateInvoiceParams ----

#[derive(Debug, Serialize)]
pub struct CreateInvoiceParams {
    /// Optional. Type of the price, can be "crypto" or "fiat". Defaults to crypto.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_currency_type")]
    pub currency_type: Option<CurrencyType>,

    /// Optional.  Required if currency_type is "crypto". Cryptocurrency alphabetic code. Supported assets: "USDT", "TON", "BTC", "ETH", "LTC", "BNB", "TRX" and "USDC".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<CryptoCurrencyCode>,

    /// Optional. Required if currency_type is "fiat". Fiat currency code. Supported fiat currencies: "USD", "EUR", "RUB", "BYN", "UAH", "GBP", "CNY", "KZT", "UZS", "GEL", "TRY", "AMD", "THB", "INR", "BRL", "IDR", "AZN", "AED", "PLN" and "ILS".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat: Option<FiatCurrencyCode>,

    /// Optional. List of cryptocurrency alphabetic codes separated comma. Assets which can be used to pay the invoice. Available only if currency_type is "crypto". Supported assets: "USDT", "TON", "BTC", "ETH", "LTC", "BNB", "TRX" and "USDC" ("JET" for testnet). Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_asset: Option<Vec<CryptoCurrencyCode>>,

    /// Amount of the invoice in float. For example: 125.50
    #[serde(serialize_with = "serialize_decimal_to_string")]
    pub amount: Decimal,

    /// Optional. Description for the invoice. User will see this description when they pay the invoice. Up to 1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional. Text of the message which will be presented to a user after the invoice is paid. Up to 2048 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_message: Option<String>,

    /// Optional. Label of the button which will be presented to a user after the invoice is paid.
    /// Supported names:
    /// viewItem – "View Item",
    /// openChannel – "View Channel",
    /// openBot – "Open Bot",
    /// callback – "Return to the bot"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_btn_name: Option<PayButtonName>,

    /// Optional. Required if paid_btn_name is specified. URL opened using the button which will be presented to a user after the invoice is paid. You can set any callback link (for example, a success link or link to homepage). Starts with https or http.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_btn_url: Option<String>,

    /// Optional. Any data you want to attach to the invoice (for example, user ID, payment ID, ect).
    /// Up to 4kb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    /// Optional. Allow a user to add a comment to the payment.
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_comments: Option<bool>,

    /// Optional. Allow a user to pay the invoice anonymously.
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous: Option<bool>,

    /// Optional. You can set a payment time limit for the invoice in seconds.
    /// Values between 1-2678400 are accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<u32>,
}

fn default_currency_type() -> Option<CurrencyType> {
    Some(CurrencyType::Crypto)
}

impl Default for CreateInvoiceParams {
    fn default() -> Self {
        Self {
            currency_type: default_currency_type(),
            asset: Some(CryptoCurrencyCode::Ton),
            fiat: None,
            accept_asset: None,
            amount: Decimal::ZERO,
            description: None,
            hidden_message: None,
            paid_btn_name: None,
            paid_btn_url: None,
            payload: None,
            allow_comments: None,
            allow_anonymous: None,
            expires_in: None,
        }
    }
}

impl FieldValidate for CreateInvoiceParams {
    fn validate(&self) -> CryptoBotResult<()> {
        // Either asset or fiat is required
        match self.currency_type {
            Some(CurrencyType::Crypto) => {
                validate_dependency!(
                    self.asset.is_none(),
                    "asset",
                    "asset is required if currency_type is crypto"
                )
            }

            Some(CurrencyType::Fiat) => {
                validate_dependency!(
                    self.fiat.is_none(),
                    "fiat",
                    "fiat is required if currency_type is fiat"
                )
            }

            None => {}
        }

        // Amount > 0
        if self.amount < Decimal::ZERO {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Amount must be greater than 0".to_string(),
                field: Some("amount".to_string()),
            });
        }

        // Description <= 1024 chars
        if let Some(desc) = &self.description {
            if desc.chars().count() > 1024 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "description_too_long".to_string(),
                    field: Some("description".to_string()),
                });
            }
        }

        // Hidden message <= 2048 chars
        if let Some(msg) = &self.hidden_message {
            if msg.chars().count() > 2048 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "hidden_message_too_long".to_string(),
                    field: Some("hidden_message".to_string()),
                });
            }
        }

        // PayBtnName & PayBtnUrl
        match (&self.paid_btn_name, &self.paid_btn_url) {
            (Some(_), None) => {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Missing,
                    message: "paid_btn_url is required when paid_btn_name is provided".to_string(),
                    field: Some("paid_btn_url".to_string()),
                });
            }
            (None, Some(_)) => {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Missing,
                    message: "paid_btn_name is required when paid_btn_url is provided".to_string(),
                    field: Some("paid_btn_name".to_string()),
                });
            }
            (Some(_), Some(_)) => {
                // PayBtnUrl must be a valid URL
                if let Some(url) = &self.paid_btn_url {
                    // TODO: maybe we need crate Url to check if it's valid
                    if !url.starts_with("https://") && !url.starts_with("http://") {
                        return Err(CryptoBotError::ValidationError {
                            kind: ValidationErrorKind::Format,
                            message: "paid_btn_url_invalid".to_string(),
                            field: Some("paid_btn_url".to_string()),
                        });
                    }
                }
            }
            _ => {}
        }

        // Payload up to 4kb
        if let Some(payload) = &self.payload {
            if payload.chars().count() > 4096 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "payload_too_long".to_string(),
                    field: Some("payload".to_string()),
                });
            }
        }

        // ExpiresIn between 1 and 2678400 seconds
        if let Some(expires_in) = &self.expires_in {
            if !(&1..=&2678400).contains(&expires_in) {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "expires_in_invalid".to_string(),
                    field: Some("expires_in".to_string()),
                });
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ContextValidate for CreateInvoiceParams {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        if let Some(asset) = &self.asset {
            validate_amount(&self.amount, asset, ctx).await?;
        }
        Ok(())
    }
}

// ---- DeleteInvoiceParams ----

#[derive(Debug, Serialize)]
pub struct DeleteInvoiceParams {
    pub invoice_id: u64,
}

impl From<u64> for DeleteInvoiceParams {
    fn from(invoice_id: u64) -> Self {
        Self { invoice_id }
    }
}

// ---- Tests ----

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_serialize_invoice_ids() {
        // Test with values
        let params = GetInvoicesParams {
            invoice_ids: Some(vec![1, 2, 3]),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["invoice_ids"], "1,2,3");

        // Test empty vector
        let params = GetInvoicesParams {
            invoice_ids: Some(vec![]),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert!(json.get("invoice_ids").is_none());

        // Test None
        let params = GetInvoicesParams {
            invoice_ids: None,
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert!(json.get("invoice_ids").is_none());
    }

    #[test]
    fn test_get_invoices_params_validation() {
        // Test invalid count
        let params = GetInvoicesParams {
            count: Some(1001),
            ..Default::default()
        };
        assert!(params.validate().is_err());
    }

    // ! Checked
    #[test]
    fn test_create_invoice_params_validation_amount() {
        let params = CreateInvoiceParams {
            amount: dec!(-1),
            ..Default::default()
        };

        let result = params.validate();

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "amount"
        ));
    }

    // ! Checked
    #[test]
    fn test_validation_currency_type_dependencies() {
        // Test crypto without asset
        let params = CreateInvoiceParams {
            currency_type: Some(CurrencyType::Crypto),
            asset: None,
            amount: dec!(10),
            ..Default::default()
        };

        let result = params.validate();
        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Missing,
                field: Some(field),
                ..
            }) if field == "asset"
        ));

        // Test fiat without fiat currency
        let params = CreateInvoiceParams {
            currency_type: Some(CurrencyType::Fiat),
            fiat: None,
            amount: dec!(10),
            ..Default::default()
        };

        let result = params.validate();
        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Missing,
                field: Some(field),
                ..
            }) if field == "fiat"
        ));
    }

    // ! Checked
    #[test]
    fn test_validation_string_lengths() {
        // Test description length
        let params = CreateInvoiceParams {
            amount: dec!(10),
            description: Some("a".repeat(1025)),
            ..Default::default()
        };

        let result = params.validate();
        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "description"
        ));

        // Test hidden message length
        let params = CreateInvoiceParams {
            amount: dec!(10),
            hidden_message: Some("a".repeat(2049)),
            ..Default::default()
        };

        let result = params.validate();
        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "hidden_message"
        ));
    }

    // ! Checked
    #[test]
    fn test_validation_paid_button() {
        // Test paid_btn_url required when paid_btn_name is set
        let params = CreateInvoiceParams {
            amount: dec!(10),
            paid_btn_name: Some(PayButtonName::ViewItem),
            paid_btn_url: None,
            ..Default::default()
        };

        let result = params.validate();

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Missing,
                field: Some(field),
                ..
            }) if field == "paid_btn_url"
        ));

        // Test invalid URL format
        let params = CreateInvoiceParams {
            amount: dec!(10),
            paid_btn_name: Some(PayButtonName::ViewItem),
            paid_btn_url: Some("invalid-url".to_string()),
            ..Default::default()
        };

        let result = params.validate();
        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Format,
                field: Some(field),
                ..
            }) if field == "paid_btn_url"
        ));
    }
}
