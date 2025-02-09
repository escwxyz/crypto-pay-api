use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    error::CryptoBotError,
    error::CryptoBotResult,
    error::ValidationErrorKind,
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
pub struct Check {
    /// Unique ID for this check.
    pub check_id: u64,

    /// Hash of the check.
    pub hash: String,

    /// Cryptocurrency alphabetic code. Currently, can be “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub asset: CryptoCurrencyCode,

    /// Amount of the check in float.
    #[serde(deserialize_with = "deserialize_decimal_from_string")]
    pub amount: Decimal,

    /// URL should be provided to the user to activate the check.
    pub bot_check_url: String,

    /// Status of the check, can be “active” or “activated”.
    pub status: CheckStatus,

    /// Date the check was created in ISO 8601 format.
    pub created_at: DateTime<Utc>,

    /// Date the check was activated in ISO 8601 format.
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Active,
    Activated,
}

// ---- CreateCheckParams ----

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCheckParams {
    /// Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    pub asset: CryptoCurrencyCode,

    /// Amount of the check in float. For example: 125.50
    #[serde(serialize_with = "serialize_decimal_to_string")]
    pub amount: Decimal,

    /// Optional. ID of the user who will be able to activate the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_to_user_id: Option<u64>,

    /// Optional. A user with the specified username will be able to activate the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_to_username: Option<String>,
}

impl FieldValidate for CreateCheckParams {
    fn validate(&self) -> CryptoBotResult<()> {
        if self.amount < Decimal::ZERO {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Amount must be greater than 0".to_string(),
                field: Some("amount".to_string()),
            });
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl ContextValidate for CreateCheckParams {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        validate_amount(&self.amount, &self.asset, ctx).await
    }
}

// ---- GetChecksParams ----

#[derive(Debug, Default, Serialize)]
pub struct GetChecksParams {
    /// Optional. Cryptocurrency alphabetic code. Supported assets: “USDT”, “TON”, “BTC”, “ETH”, “LTC”, “BNB”, “TRX” and “USDC” (and “JET” for testnet).
    /// Defaults to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<CryptoCurrencyCode>,

    /// Optional. List of check IDs separated by comma.
    #[serde(
        serialize_with = "serialize_comma_separated_list",
        skip_serializing_if = "GetChecksParams::should_skip_check_ids"
    )]
    pub check_ids: Option<Vec<u64>>,

    /// Optional. Status of check to be returned. Available statuses: “active” and “activated”.
    /// Defaults to all statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckStatus>,

    /// Optional. Offset needed to return a specific subset of check.
    /// Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Optional. Number of check to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
}

impl GetChecksParams {
    fn should_skip_check_ids(check_ids: &Option<Vec<u64>>) -> bool {
        !matches!(check_ids, Some(check_ids) if !check_ids.is_empty())
    }
}

impl FieldValidate for GetChecksParams {
    fn validate(&self) -> CryptoBotResult<()> {
        if let Some(count) = &self.count {
            validate_count(*count)?;
        }
        Ok(())
    }
}

// ---- DeleteCheckParams ----
#[derive(Debug, Serialize)]
pub struct DeleteCheckParams {
    pub check_id: u64,
}

impl From<u64> for DeleteCheckParams {
    fn from(check_id: u64) -> Self {
        Self { check_id }
    }
}

#[cfg(test)]
mod tests {

    use rust_decimal_macros::dec;

    use crate::{error::CryptoBotError, error::ValidationErrorKind};

    use super::*;

    // ! Checked
    #[test]
    fn test_create_check_params_validation_amount() {
        let params = CreateCheckParams {
            asset: CryptoCurrencyCode::Ton,
            amount: dec!(-1),
            pin_to_user_id: None,
            pin_to_username: None,
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
    fn test_get_checks_params_validate() {
        let params = GetChecksParams {
            count: Some(1001),
            ..Default::default()
        };

        let result = params.validate();

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "count"
        ));
    }
}
