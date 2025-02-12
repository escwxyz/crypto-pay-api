use std::marker::PhantomData;

use rust_decimal::Decimal;

use crate::{
    api::ExchangeRateAPI,
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    models::{CryptoCurrencyCode, Missing, Set},
    validation::{
        validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext,
    },
};

use super::params::{GetTransfersParams, TransferParams};

/* #region GetTransfersParamsBuilder */

#[derive(Debug, Default)]
pub struct GetTransfersParamsBuilder {
    asset: Option<CryptoCurrencyCode>,
    transfer_ids: Option<Vec<u64>>,
    spend_id: Option<String>,
    offset: Option<u32>,
    count: Option<u16>,
}

impl GetTransfersParamsBuilder {
    /// Create a new `GetTransfersParamsBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the asset for the transfers.
    /// Optional. Defaults to all currencies.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> Self {
        self.asset = Some(asset);
        self
    }

    /// Set the transfer IDs for the transfers.
    /// Optional.
    pub fn transfer_ids(mut self, ids: Vec<u64>) -> Self {
        self.transfer_ids = Some(ids);
        self
    }

    /// Set the spend ID for the transfers.
    /// Optional. Unique UTF-8 transfer string.
    pub fn spend_id(mut self, spend_id: impl Into<String>) -> Self {
        self.spend_id = Some(spend_id.into());
        self
    }

    /// Set the offset for the transfers.
    /// Optional. Offset needed to return a specific subset of transfers.
    /// Defaults to 0.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the count for the transfers.
    /// Optional. Defaults to 100. Values between 1-1000 are accepted.
    pub fn count(mut self, count: u16) -> Self {
        self.count = Some(count);
        self
    }
}

impl FieldValidate for GetTransfersParamsBuilder {
    fn validate(&self) -> CryptoBotResult<()> {
        if let Some(count) = &self.count {
            validate_count(*count)?;
        }
        Ok(())
    }
}

impl GetTransfersParamsBuilder {
    pub fn build(self) -> CryptoBotResult<GetTransfersParams> {
        self.validate()?;

        Ok(GetTransfersParams::new(
            self.asset,
            self.transfer_ids,
            self.spend_id,
            self.offset,
            self.count,
        ))
    }
}

/* #endregion */

/* #region TransferParamsBuilder */

#[derive(Debug)]
pub struct TransferParamsBuilder<U = Missing, A = Missing, M = Missing, S = Missing> {
    user_id: u64,
    asset: CryptoCurrencyCode,
    amount: Decimal,
    spend_id: String,
    comment: Option<String>,
    disable_send_notification: Option<bool>,
    _state: PhantomData<(U, A, M, S)>,
}

impl TransferParamsBuilder<Missing, Missing, Missing, Missing> {
    /// Create a new `TransferParamsBuilder` with default values.
    pub fn new() -> TransferParamsBuilder<Missing, Missing, Missing, Missing> {
        Self {
            user_id: 0,
            asset: CryptoCurrencyCode::Ton,
            amount: Decimal::ZERO,
            spend_id: String::new(),
            comment: None,
            disable_send_notification: None,
            _state: PhantomData,
        }
    }
}

impl Default for TransferParamsBuilder<Missing, Missing, Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A, M, S> TransferParamsBuilder<Missing, A, M, S> {
    /// Set the Telegram user ID for the transfer.
    pub fn user_id(mut self, user_id: u64) -> TransferParamsBuilder<Set, A, M, S> {
        self.user_id = user_id;
        self.transform()
    }
}

impl<U, M, S> TransferParamsBuilder<U, Missing, M, S> {
    /// Set the asset for the transfer.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> TransferParamsBuilder<U, Set, M, S> {
        self.asset = asset;
        self.transform()
    }
}

impl<U, A, S> TransferParamsBuilder<U, A, Missing, S> {
    /// Set the amount for the transfer.
    /// The minimum and maximum amount limits for each of the supported assets roughly correspond to 1-25000 USD.
    pub fn amount(mut self, amount: Decimal) -> TransferParamsBuilder<U, A, Set, S> {
        self.amount = amount;
        self.transform()
    }
}

impl<U, A, M> TransferParamsBuilder<U, A, M, Missing> {
    /// Set the spend ID for the transfer.
    /// Random UTF-8 string unique per transfer for idempotent requests.
    /// The same spend_id can be accepted only once from your app.
    /// Up to 64 symbols.
    pub fn spend_id(mut self, spend_id: impl Into<String>) -> TransferParamsBuilder<U, A, M, Set> {
        self.spend_id = spend_id.into();
        self.transform()
    }
}

impl<U, A, M, S> TransferParamsBuilder<U, A, M, S> {
    /// Set the comment for the transfer.
    /// Optional. Comment for the transfer.
    /// Users will see this comment in the notification about the transfer.
    /// Up to 1024 symbols.
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Set the disable send notification for the transfer.
    /// Optional. Pass true to not send to the user the notification about the transfer.
    /// Defaults to false.
    pub fn disable_send_notification(mut self, disable: bool) -> Self {
        self.disable_send_notification = Some(disable);
        self
    }

    fn transform<U2, A2, M2, S2>(self) -> TransferParamsBuilder<U2, A2, M2, S2> {
        TransferParamsBuilder {
            user_id: self.user_id,
            asset: self.asset,
            amount: self.amount,
            spend_id: self.spend_id,
            comment: self.comment,
            disable_send_notification: self.disable_send_notification,
            _state: PhantomData,
        }
    }
}

impl FieldValidate for TransferParamsBuilder<Set, Set, Set, Set> {
    fn validate(&self) -> CryptoBotResult<()> {
        if self.spend_id.chars().count() > 64 {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Spend ID must be less than 64 symbols".to_string(),
                field: Some("spend_id".to_string()),
            });
        }

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
impl ContextValidate for TransferParamsBuilder<Set, Set, Set, Set> {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        validate_amount(&self.amount, &self.asset, ctx).await
    }
}

impl TransferParamsBuilder<Set, Set, Set, Set> {
    pub async fn build(self, client: &CryptoBot) -> CryptoBotResult<TransferParams> {
        self.validate()?;

        let rates = client.get_exchange_rates().await?;

        let ctx = ValidationContext {
            exchange_rates: rates,
        };

        self.validate_with_context(&ctx).await?;

        Ok(TransferParams {
            user_id: self.user_id,
            asset: self.asset,
            amount: self.amount,
            spend_id: self.spend_id,
            comment: self.comment,
            disable_send_notification: self.disable_send_notification,
        })
    }
}

/* #endregion */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_transfers_params() {
        let params = GetTransfersParamsBuilder::new()
            .asset(CryptoCurrencyCode::Ton)
            .offset(2)
            .spend_id("spend_id")
            .build()
            .unwrap();

        assert_eq!(params.asset, Some(CryptoCurrencyCode::Ton));
        assert_eq!(params.offset, Some(2));
        assert_eq!(params.spend_id, Some("spend_id".to_string()));
    }

    #[test]
    fn test_get_transfers_params_invalid_count() {
        let params = GetTransfersParamsBuilder::new().count(1001).build();

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
    async fn test_transfer_params() {
        let client = CryptoBot::test_client();

        let params = TransferParamsBuilder::new()
            .user_id(123456789)
            .asset(CryptoCurrencyCode::Ton)
            .amount(Decimal::from(100))
            .spend_id("test_id")
            .comment("test comment")
            .disable_send_notification(true)
            .build(&client)
            .await
            .unwrap();

        assert_eq!(params.user_id, 123456789);
        assert_eq!(params.asset, CryptoCurrencyCode::Ton);
        assert_eq!(params.amount, Decimal::from(100));
        assert_eq!(params.spend_id, "test_id");
        assert_eq!(params.comment, Some("test comment".to_string()));
        assert_eq!(params.disable_send_notification, Some(true));
    }
    #[tokio::test]
    async fn test_transfer_params_invalid_spend_id() {
        let client = CryptoBot::test_client();

        let result = TransferParamsBuilder::default()
            .user_id(123456789)
            .asset(CryptoCurrencyCode::Ton)
            .amount(Decimal::from(100))
            .spend_id("x".repeat(65))
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "spend_id"
        ));
    }

    #[tokio::test]
    async fn test_transfer_params_validate_amount() {
        let client = CryptoBot::test_client();

        let result = TransferParamsBuilder::new()
            .user_id(123456789)
            .asset(CryptoCurrencyCode::Ton)
            .amount(Decimal::from(100000))
            .spend_id("test_spend_id")
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

    #[tokio::test]
    async fn test_transfer_params_validate_comments() {
        let client = CryptoBot::test_client();

        let result = TransferParamsBuilder::new()
            .user_id(123456789)
            .asset(CryptoCurrencyCode::Ton)
            .amount(Decimal::from(100))
            .spend_id("test_spend_id")
            .comment("x".repeat(1025))
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "comment"
        ));
    }
}
