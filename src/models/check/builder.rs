use std::marker::PhantomData;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    api::ExchangeRateAPI,
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    models::{CryptoCurrencyCode, Missing, Set},
    validation::{validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext},
};

use super::{CheckStatus, CreateCheckParams, GetChecksParams};

/* #region CreateCheckParamsBuilder */

pub struct CreateCheckParamsBuilder<A = Missing, M = Missing> {
    asset: CryptoCurrencyCode,
    amount: Decimal,
    pin_to_user_id: Option<u64>,
    pin_to_username: Option<String>,
    _state: PhantomData<(A, M)>,
}

impl Default for CreateCheckParamsBuilder {
    fn default() -> Self {
        Self {
            asset: CryptoCurrencyCode::Ton,
            amount: dec!(0),
            pin_to_user_id: None,
            pin_to_username: None,
            _state: PhantomData,
        }
    }
}

impl CreateCheckParamsBuilder {
    /// Create a new `CreateCheckParamsBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<M> CreateCheckParamsBuilder<Missing, M> {
    /// Set the asset for the check.
    /// Cryptocurrency alphabetic code.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> CreateCheckParamsBuilder<Set, M> {
        self.asset = asset;
        self.transform()
    }
}

impl<A> CreateCheckParamsBuilder<A, Missing> {
    /// Set the amount for the check.
    /// Amount of the check in float.
    pub fn amount(mut self, amount: Decimal) -> CreateCheckParamsBuilder<A, Set> {
        self.amount = amount;
        self.transform()
    }
}

impl<A, M> CreateCheckParamsBuilder<A, M> {
    /// Set the user ID to pin the check to.
    /// Optional. ID of the user who will be able to activate the check.
    pub fn pin_to_user_id(mut self, pin_to_user_id: u64) -> Self {
        self.pin_to_user_id = Some(pin_to_user_id);
        self.transform()
    }

    /// Set the username to pin the check to.
    /// Optional. A user with the specified username will be able to activate the check.
    pub fn pin_to_username(mut self, pin_to_username: &str) -> Self {
        self.pin_to_username = Some(pin_to_username.to_string());
        self
    }

    fn transform<A2, M2>(self) -> CreateCheckParamsBuilder<A2, M2> {
        CreateCheckParamsBuilder {
            asset: self.asset,
            amount: self.amount,
            pin_to_user_id: self.pin_to_user_id,
            pin_to_username: self.pin_to_username,
            _state: PhantomData,
        }
    }
}

impl FieldValidate for CreateCheckParamsBuilder<Set, Set> {
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
impl ContextValidate for CreateCheckParamsBuilder<Set, Set> {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        validate_amount(&self.amount, &self.asset, ctx).await
    }
}

impl CreateCheckParamsBuilder<Set, Set> {
    pub async fn build(self, client: &CryptoBot) -> CryptoBotResult<CreateCheckParams> {
        self.validate()?;

        let exchange_rates = client.get_exchange_rates().await?;

        let ctx = ValidationContext { exchange_rates };

        self.validate_with_context(&ctx).await?;

        Ok(CreateCheckParams {
            asset: self.asset,
            amount: self.amount,
            pin_to_user_id: self.pin_to_user_id,
            pin_to_username: self.pin_to_username,
        })
    }
}

/* #endregion */

/* #region GetChecksParamsBuilder */

#[derive(Debug, Default)]
pub struct GetChecksParamsBuilder {
    asset: Option<CryptoCurrencyCode>,
    check_ids: Option<Vec<u64>>,
    status: Option<CheckStatus>,
    offset: Option<u32>,
    count: Option<u16>,
}

impl GetChecksParamsBuilder {
    /// Create a new `GetChecksParamsBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the asset for the checks.
    /// Optional. Defaults to all currencies.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> Self {
        self.asset = Some(asset);
        self
    }

    /// Set the check IDs for the checks.
    pub fn check_ids(mut self, check_ids: Vec<u64>) -> Self {
        self.check_ids = Some(check_ids);
        self
    }

    /// Set the status for the checks.
    /// Optional. Status of check to be returned.
    /// Defaults to all statuses.
    pub fn status(mut self, status: CheckStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the offset for the checks.
    /// Optional. Offset needed to return a specific subset of check.
    /// Defaults to 0.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the count for the checks.
    /// Optional. Number of check to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    pub fn count(mut self, count: u16) -> Self {
        self.count = Some(count);
        self
    }
}

impl FieldValidate for GetChecksParamsBuilder {
    fn validate(&self) -> CryptoBotResult<()> {
        if let Some(count) = &self.count {
            validate_count(*count)?;
        }
        Ok(())
    }
}

impl GetChecksParamsBuilder {
    pub fn build(self) -> CryptoBotResult<GetChecksParams> {
        self.validate()?;

        Ok(GetChecksParams {
            asset: self.asset,
            check_ids: self.check_ids,
            status: self.status,
            offset: self.offset,
            count: self.count,
        })
    }
}
/* #endregion */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_checks_params_builder() {
        let params = GetChecksParamsBuilder::new()
            .count(5)
            .asset(CryptoCurrencyCode::Ton)
            .status(CheckStatus::Activated)
            .offset(10)
            .build()
            .unwrap();
        assert_eq!(params.count, Some(5));
        assert_eq!(params.asset, Some(CryptoCurrencyCode::Ton));
        assert_eq!(params.status, Some(CheckStatus::Activated));
        assert_eq!(params.offset, Some(10));
    }

    #[test]
    fn test_get_checks_params_builder_invalid_count() {
        let params = GetChecksParamsBuilder::new().count(1001).build();
        assert!(matches!(
            params,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "count"
        ));
    }

    #[tokio::test]
    async fn test_create_check_params_builder() {
        let client = CryptoBot::test_client();

        let params = CreateCheckParamsBuilder::new()
            .asset(CryptoCurrencyCode::Ton)
            .amount(Decimal::from(100))
            .pin_to_user_id(123456789)
            .pin_to_username("test_username")
            .build(&client)
            .await
            .unwrap();

        assert_eq!(params.asset, CryptoCurrencyCode::Ton);
        assert_eq!(params.amount, Decimal::from(100));
        assert_eq!(params.pin_to_user_id, Some(123456789));
        assert_eq!(params.pin_to_username, Some("test_username".to_string()));
    }

    #[tokio::test]
    async fn test_create_check_params_builder_invalid_amount() {
        let client = CryptoBot::test_client();
        let result = CreateCheckParamsBuilder::new()
            .asset(CryptoCurrencyCode::Ton)
            .amount(Decimal::from(-100))
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "amount"
        ));
    }
}
