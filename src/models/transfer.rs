use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    utils::{
        deserialize_decimal_from_string, serialize_comma_separated_list,
        serialize_decimal_to_string,
    },
    validation::{
        validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext,
    },
};

use super::CryptoCurrencyCode;

#[derive(Debug, Deserialize)]
pub struct Transfer {
    /// Unique ID for this transfer.
    pub transfer_id: u64,

    /// Unique UTF-8 string.
    pub spend_id: String,

    /// Telegram user ID the transfer was sent to.
    pub user_id: u64,

    /// Cryptocurrency alphabetic code. Currently, can be “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub asset: CryptoCurrencyCode,

    /// Amount of the transfer in float.
    #[serde(deserialize_with = "deserialize_decimal_from_string")]
    pub amount: Decimal,

    /// Status of the transfer, can only be “completed”.
    pub status: TransferStatus,

    /// Date the transfer was completed in ISO 8601 format.
    pub completed_at: DateTime<Utc>,

    /// Optional. Comment for this transfer.
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransferStatus {
    Completed,
}

// ---- TransferParams ----
#[derive(Debug, Serialize)]
pub struct TransferParams {
    /// User ID in Telegram. User must have previously used @CryptoBot (@CryptoTestnetBot for testnet).
    pub user_id: u64,

    /// Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub asset: CryptoCurrencyCode,

    /// Amount of the transfer in float.
    /// The minimum and maximum amount limits for each of the supported assets roughly correspond to 1-25000 USD.
    /// Use getExchangeRates to convert amounts. For example: 125.50
    #[serde(serialize_with = "serialize_decimal_to_string")]
    pub amount: Decimal,

    /// Random UTF-8 string unique per transfer for idempotent requests.
    /// The same spend_id can be accepted only once from your app.
    /// Up to 64 symbols.
    pub spend_id: String, // ? what is it? like order or invoice id?

    /// Optional. Comment for the transfer.
    /// Users will see this comment in the notification about the transfer.
    /// Up to 1024 symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Optional. Pass true to not send to the user the notification about the transfer.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_send_notification: Option<bool>,
}

impl FieldValidate for TransferParams {
    fn validate(&self) -> CryptoBotResult<()> {
        // spend id
        if self.spend_id.chars().count() > 64 {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Spend ID must be less than 64 symbols".to_string(),
                field: Some("spend_id".to_string()),
            });
        }

        // comment
        if let Some(comment) = &self.comment {
            if comment.chars().count() > 1024 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "Comment must be less than 1024 symbols".to_string(),
                    field: Some("comment".to_string()),
                });
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl ContextValidate for TransferParams {
    async fn validate_with_context(&self, context: &ValidationContext) -> CryptoBotResult<()> {
        validate_amount(&self.amount, &self.asset, context).await
    }
}

// ---- GetTransfersParams ----

#[derive(Debug, Default, Serialize)]
pub struct GetTransfersParams {
    /// Optional. Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<CryptoCurrencyCode>,

    /// Optional. List of transfer IDs separated by comma.
    #[serde(
        serialize_with = "serialize_comma_separated_list",
        skip_serializing_if = "GetTransfersParams::should_skip_transfer_ids"
    )]
    pub transfer_ids: Option<Vec<u64>>,

    /// Optional. Unique UTF-8 transfer string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_id: Option<String>,

    /// Optional. Offset needed to return a specific subset of transfers.
    /// Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Optional. Number of transfers to be returned.
    /// Values between 1-1000 are accepted.
    /// Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
}

impl GetTransfersParams {
    fn should_skip_transfer_ids(ids: &Option<Vec<u64>>) -> bool {
        !matches!(ids, Some(ids) if !ids.is_empty())
    }
}

impl FieldValidate for GetTransfersParams {
    fn validate(&self) -> CryptoBotResult<()> {
        if let Some(count) = &self.count {
            validate_count(*count)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ! Checked
    #[test]
    fn test_transfer_params_validate() {
        let params = TransferParams {
            user_id: 123456789,
            asset: CryptoCurrencyCode::Ton,
            amount: Decimal::from(100),
            spend_id: "test_spend_id".to_string(),
            comment: Some("test_comment".to_string()),
            disable_send_notification: Some(true),
        };

        let result = params.validate();
        assert!(result.is_ok());
    }

    // TODO: Add test for amount validation
    // #[test]
    // fn test_transfer_params_validate_amount() {
    //     let params = TransferParams {
    //         user_id: 123456789,
    //         asset: CryptoCurrencyCode::Ton,
    //         amount: Decimal::from(100000),
    //         spend_id: "test_spend_id".to_string(),
    //         comment: Some("test_comment".to_string()),
    //         disable_send_notification: Some(true),
    //     };

    //     let result = params.validate();
    //     assert!(result.is_err());
    // }

    // ! Checked
    #[test]
    fn test_transfer_params_spend_id() {
        let params = TransferParams {
            user_id: 123456789,
            asset: CryptoCurrencyCode::Ton,
            amount: Decimal::from(100),
            spend_id: "test_spend_id_1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string(),
            comment: Some("test_comment".to_string()),
            disable_send_notification: Some(true),
        };

        let result = params.validate();
        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "spend_id"
        ));
    }
}
