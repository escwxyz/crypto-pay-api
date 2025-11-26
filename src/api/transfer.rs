use async_trait::async_trait;
use std::marker::PhantomData;

use rust_decimal::Decimal;

use crate::{
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    models::{
        APIEndpoint, APIMethod, CryptoCurrencyCode, GetTransfersParams, GetTransfersResponse, Method, Missing, Set,
        Transfer, TransferParams,
    },
    validation::{validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext},
};

use super::TransferAPI;
use crate::api::ExchangeRateAPI;

pub struct GetTransfersBuilder<'a> {
    client: &'a CryptoBot,
    params: GetTransfersParams,
}

impl<'a> GetTransfersBuilder<'a> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self {
            client,
            params: GetTransfersParams::default(),
        }
    }

    /// Set the asset for the transfers.
    /// Optional. Defaults to all currencies.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> Self {
        self.params.asset = Some(asset);
        self
    }

    /// Set the transfer IDs for the transfers.
    /// Optional.
    pub fn transfer_ids(mut self, ids: Vec<u64>) -> Self {
        self.params.transfer_ids = Some(ids);
        self
    }

    /// Set the spend ID for the transfers.
    /// Optional. Unique UTF-8 transfer string.
    pub fn spend_id(mut self, spend_id: impl Into<String>) -> Self {
        self.params.spend_id = Some(spend_id.into());
        self
    }

    /// Set the offset for the transfers.
    /// Optional. Offset needed to return a specific subset of transfers.
    /// Defaults to 0.
    pub fn offset(mut self, offset: u32) -> Self {
        self.params.offset = Some(offset);
        self
    }

    /// Set the count for the transfers.
    /// Optional. Defaults to 100. Values between 1-1000 are accepted.
    pub fn count(mut self, count: u16) -> Self {
        self.params.count = Some(count);
        self
    }

    /// Executes the request to get transfers
    pub async fn execute(self) -> CryptoBotResult<Vec<Transfer>> {
        if let Some(count) = self.params.count {
            validate_count(count)?;
        }

        let response: GetTransfersResponse = self
            .client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetTransfers,
                    method: Method::GET,
                },
                Some(&self.params),
            )
            .await?;

        Ok(response.items)
    }
}

pub struct TransferBuilder<'a, U = Missing, A = Missing, M = Missing, S = Missing> {
    client: &'a CryptoBot,
    user_id: u64,
    asset: CryptoCurrencyCode,
    amount: Decimal,
    spend_id: String,
    comment: Option<String>,
    disable_send_notification: Option<bool>,
    _state: PhantomData<(U, A, M, S)>,
}

impl<'a> TransferBuilder<'a, Missing, Missing, Missing, Missing> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self {
            client,
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

impl<'a, A, M, S> TransferBuilder<'a, Missing, A, M, S> {
    /// Set the Telegram user ID for the transfer.
    pub fn user_id(mut self, user_id: u64) -> TransferBuilder<'a, Set, A, M, S> {
        self.user_id = user_id;
        self.transform()
    }
}

impl<'a, U, M, S> TransferBuilder<'a, U, Missing, M, S> {
    /// Set the asset for the transfer.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> TransferBuilder<'a, U, Set, M, S> {
        self.asset = asset;
        self.transform()
    }
}

impl<'a, U, A, S> TransferBuilder<'a, U, A, Missing, S> {
    /// Set the amount for the transfer.
    /// The minimum and maximum amount limits for each of the supported assets roughly correspond to 1-25000 USD.
    pub fn amount(mut self, amount: Decimal) -> TransferBuilder<'a, U, A, Set, S> {
        self.amount = amount;
        self.transform()
    }
}

impl<'a, U, A, M> TransferBuilder<'a, U, A, M, Missing> {
    /// Set the spend ID for the transfer.
    /// Random UTF-8 string unique per transfer for idempotent requests.
    /// The same spend_id can be accepted only once from your app.
    /// Up to 64 symbols.
    pub fn spend_id(mut self, spend_id: impl Into<String>) -> TransferBuilder<'a, U, A, M, Set> {
        self.spend_id = spend_id.into();
        self.transform()
    }
}

impl<'a, U, A, M, S> TransferBuilder<'a, U, A, M, S> {
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

    fn transform<U2, A2, M2, S2>(self) -> TransferBuilder<'a, U2, A2, M2, S2> {
        TransferBuilder {
            client: self.client,
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

impl<'a> FieldValidate for TransferBuilder<'a, Set, Set, Set, Set> {
    fn validate(&self) -> CryptoBotResult<()> {
        if self.spend_id.chars().count() > 64 {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Spend ID must be at most 64 symbols".to_string(),
                field: Some("spend_id".to_string()),
            });
        }

        if let Some(comment) = &self.comment {
            if comment.chars().count() > 1024 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "Comment must be at most 1024 symbols".to_string(),
                    field: Some("comment".to_string()),
                });
            }
        }

        Ok(())
    }
}

#[async_trait]
impl<'a> ContextValidate for TransferBuilder<'a, Set, Set, Set, Set> {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        validate_amount(&self.amount, &self.asset, ctx).await
    }
}

impl<'a> TransferBuilder<'a, Set, Set, Set, Set> {
    /// Executes the request to transfer cryptocurrency
    pub async fn execute(self) -> CryptoBotResult<Transfer> {
        self.validate()?;

        let rates = self.client.get_exchange_rates().execute().await?;
        let ctx = ValidationContext { exchange_rates: rates };
        self.validate_with_context(&ctx).await?;

        let params = TransferParams {
            user_id: self.user_id,
            asset: self.asset,
            amount: self.amount,
            spend_id: self.spend_id,
            comment: self.comment,
            disable_send_notification: self.disable_send_notification,
        };

        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::Transfer,
                    method: Method::POST,
                },
                Some(&params),
            )
            .await
    }
}

#[async_trait]
impl TransferAPI for CryptoBot {
    /// Transfer cryptocurrency to a user
    ///
    /// # Returns
    /// * `TransferBuilder` - A builder to construct the transfer parameters
    fn transfer(&self) -> TransferBuilder<'_> {
        TransferBuilder::new(self)
    }

    /// Get transfers history
    ///
    /// # Returns
    /// * `GetTransfersBuilder` - A builder to construct the filter parameters
    fn get_transfers(&self) -> GetTransfersBuilder<'_> {
        GetTransfersBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use rust_decimal_macros::dec;
    use serde_json::json;

    use crate::{
        api::TransferAPI,
        client::CryptoBot,
        models::{CryptoCurrencyCode, TransferStatus},
        utils::test_utils::TestContext,
    };

    impl TestContext {
        pub fn mock_transfer_response(&mut self) -> Mock {
            self.server
                .mock("POST", "/transfer")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "transfer_id": 1,
                            "user_id": 123456789,
                            "asset": "TON",
                            "amount": "10.5",
                            "status": "completed",
                            "completed_at": "2024-03-14T12:00:00Z",
                            "comment": "test_comment",
                            "spend_id": "test_spend_id",
                            "disable_send_notification": false,
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_transfers_response_without_params(&mut self) -> Mock {
            self.server
                .mock("GET", "/getTransfers")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "items": [{
                                "transfer_id": 1,
                                "user_id": 123456789,
                                "asset": "TON",
                                "amount": "10.5",
                                "status": "completed",
                                "completed_at": "2024-03-14T12:00:00Z",
                                "comment": "test_comment",
                                "spend_id": "test_spend_id",
                                "disable_send_notification": false,
                            }]
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_transfers_response_with_transfer_ids(&mut self) -> Mock {
            self.server
                .mock("GET", "/getTransfers")
                .match_body(json!({ "transfer_ids": "1" }).to_string().as_str())
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "items": [
                                {
                                    "transfer_id": 1,
                                    "user_id": 123456789,
                                    "asset": "TON",
                                    "amount": "10.5",
                                    "status": "completed",
                                    "completed_at": "2024-03-14T12:00:00Z",
                                    "comment": "test_comment",
                                    "spend_id": "test_spend_id",
                                    "disable_send_notification": false,
                                }
                            ]
                        }
                    })
                    .to_string(),
                )
                .create()
        }
    }

    #[test]
    fn test_transfer() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_transfer_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .transfer()
                .user_id(123456789)
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(10.5))
                .spend_id("test_spend_id".to_string())
                .comment("test_comment".to_string())
                .execute()
                .await
        });

        println!("result:{:?}", result);

        assert!(result.is_ok());

        let transfer = result.unwrap();
        assert_eq!(transfer.transfer_id, 1);
        assert_eq!(transfer.user_id, 123456789);
        assert_eq!(transfer.asset, CryptoCurrencyCode::Ton);
        assert_eq!(transfer.amount, dec!(10.5));
        assert_eq!(transfer.status, TransferStatus::Completed);
    }

    #[test]
    fn test_get_transfers_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_transfers_response_without_params();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_transfers().execute().await });

        assert!(result.is_ok());
        let transfers = result.unwrap();
        assert_eq!(transfers.len(), 1);

        let transfer = &transfers[0];
        assert_eq!(transfer.transfer_id, 1);
        assert_eq!(transfer.asset, CryptoCurrencyCode::Ton);
        assert_eq!(transfer.status, TransferStatus::Completed);
    }

    #[test]
    fn test_get_transfers_with_transfer_ids() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_transfers_response_with_transfer_ids();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_transfers().transfer_ids(vec![1]).execute().await });

        assert!(result.is_ok());
        let transfers = result.unwrap();
        assert_eq!(transfers.len(), 1);
    }
}
