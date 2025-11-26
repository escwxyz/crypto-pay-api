use async_trait::async_trait;
use std::marker::PhantomData;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    models::{
        APIEndpoint, APIMethod, Check, CheckStatus, CreateCheckParams, CryptoCurrencyCode, DeleteCheckParams,
        GetChecksParams, GetChecksResponse, Method, Missing, Set,
    },
    validation::{validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext},
};

use super::CheckAPI;
use crate::api::ExchangeRateAPI;

pub struct DeleteCheckBuilder<'a> {
    client: &'a CryptoBot,
    check_id: u64,
}

impl<'a> DeleteCheckBuilder<'a> {
    pub fn new(client: &'a CryptoBot, check_id: u64) -> Self {
        Self { client, check_id }
    }

    /// Executes the request to delete the check
    pub async fn execute(self) -> CryptoBotResult<bool> {
        let params = DeleteCheckParams {
            check_id: self.check_id,
        };

        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::DeleteCheck,
                    method: Method::DELETE,
                },
                Some(&params),
            )
            .await
    }
}

pub struct GetChecksBuilder<'a> {
    client: &'a CryptoBot,
    params: GetChecksParams,
}

impl<'a> GetChecksBuilder<'a> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self {
            client,
            params: GetChecksParams::default(),
        }
    }

    /// Set the asset for the checks.
    /// Optional. Defaults to all currencies.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> Self {
        self.params.asset = Some(asset);
        self
    }

    /// Set the check IDs for the checks.
    pub fn check_ids(mut self, check_ids: Vec<u64>) -> Self {
        self.params.check_ids = Some(check_ids);
        self
    }

    /// Set the status for the checks.
    /// Optional. Status of check to be returned.
    /// Defaults to all statuses.
    pub fn status(mut self, status: CheckStatus) -> Self {
        self.params.status = Some(status);
        self
    }

    /// Set the offset for the checks.
    /// Optional. Offset needed to return a specific subset of check.
    /// Defaults to 0.
    pub fn offset(mut self, offset: u32) -> Self {
        self.params.offset = Some(offset);
        self
    }

    /// Set the count for the checks.
    /// Optional. Number of check to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    pub fn count(mut self, count: u16) -> Self {
        self.params.count = Some(count);
        self
    }

    /// Executes the request to get checks
    pub async fn execute(self) -> CryptoBotResult<Vec<Check>> {
        if let Some(count) = self.params.count {
            validate_count(count)?;
        }

        let response: GetChecksResponse = self
            .client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetChecks,
                    method: Method::GET,
                },
                Some(&self.params),
            )
            .await?;

        Ok(response.items)
    }
}

pub struct CreateCheckBuilder<'a, A = Missing, M = Missing> {
    client: &'a CryptoBot,
    asset: CryptoCurrencyCode,
    amount: Decimal,
    pin_to_user_id: Option<u64>,
    pin_to_username: Option<String>,
    _state: PhantomData<(A, M)>,
}

impl<'a> CreateCheckBuilder<'a, Missing, Missing> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self {
            client,
            asset: CryptoCurrencyCode::Ton,
            amount: dec!(0),
            pin_to_user_id: None,
            pin_to_username: None,
            _state: PhantomData,
        }
    }
}

impl<'a, M> CreateCheckBuilder<'a, Missing, M> {
    /// Set the asset for the check.
    /// Cryptocurrency alphabetic code.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> CreateCheckBuilder<'a, Set, M> {
        self.asset = asset;
        self.transform()
    }
}

impl<'a, A> CreateCheckBuilder<'a, A, Missing> {
    /// Set the amount for the check.
    /// Amount of the check in float.
    pub fn amount(mut self, amount: Decimal) -> CreateCheckBuilder<'a, A, Set> {
        self.amount = amount;
        self.transform()
    }
}

impl<'a, A, M> CreateCheckBuilder<'a, A, M> {
    /// Set the user ID to pin the check to.
    /// Optional. ID of the user who will be able to activate the check.
    pub fn pin_to_user_id(mut self, pin_to_user_id: u64) -> Self {
        self.pin_to_user_id = Some(pin_to_user_id);
        self
    }

    /// Set the username to pin the check to.
    /// Optional. A user with the specified username will be able to activate the check.
    pub fn pin_to_username(mut self, pin_to_username: &str) -> Self {
        self.pin_to_username = Some(pin_to_username.to_string());
        self
    }

    fn transform<A2, M2>(self) -> CreateCheckBuilder<'a, A2, M2> {
        CreateCheckBuilder {
            client: self.client,
            asset: self.asset,
            amount: self.amount,
            pin_to_user_id: self.pin_to_user_id,
            pin_to_username: self.pin_to_username,
            _state: PhantomData,
        }
    }
}

impl<'a> FieldValidate for CreateCheckBuilder<'a, Set, Set> {
    fn validate(&self) -> CryptoBotResult<()> {
        if self.amount <= Decimal::ZERO {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Amount must be greater than 0".to_string(),
                field: Some("amount".to_string()),
            });
        }
        Ok(())
    }
}

#[async_trait]
impl<'a> ContextValidate for CreateCheckBuilder<'a, Set, Set> {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        validate_amount(&self.amount, &self.asset, ctx).await
    }
}

impl<'a> CreateCheckBuilder<'a, Set, Set> {
    /// Executes the request to create the check
    pub async fn execute(self) -> CryptoBotResult<Check> {
        self.validate()?;

        let exchange_rates = self.client.get_exchange_rates().execute().await?;
        let ctx = ValidationContext { exchange_rates };
        self.validate_with_context(&ctx).await?;

        let params = CreateCheckParams {
            asset: self.asset,
            amount: self.amount,
            pin_to_user_id: self.pin_to_user_id,
            pin_to_username: self.pin_to_username,
        };

        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::CreateCheck,
                    method: Method::POST,
                },
                Some(&params),
            )
            .await
    }
}

#[async_trait]
impl CheckAPI for CryptoBot {
    /// Creates a new cryptocurrency check
    ///
    /// A check is a unique link that can be used once to transfer cryptocurrency.
    /// Anyone who opens the link first can activate the check and get the cryptocurrency.
    ///
    /// # Returns
    /// * `CreateCheckBuilder` - A builder to construct the check parameters
    fn create_check(&self) -> CreateCheckBuilder<'_> {
        CreateCheckBuilder::new(self)
    }

    fn delete_check(&self, check_id: u64) -> DeleteCheckBuilder<'_> {
        DeleteCheckBuilder::new(self, check_id)
    }

    /// Gets a list of created cryptocurrency checks
    ///
    /// Retrieves all checks matching the specified filter parameters.
    /// If no parameters are provided, returns all checks.
    ///
    /// # Returns
    /// * `GetChecksBuilder` - A builder to construct the filter parameters
    fn get_checks(&self) -> GetChecksBuilder<'_> {
        GetChecksBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use mockito::{Matcher, Mock};
    use rust_decimal_macros::dec;
    use serde_json::json;

    use crate::{models::CryptoCurrencyCode, utils::test_utils::TestContext};

    use super::*;

    impl TestContext {
        pub fn mock_create_check_response(&mut self) -> Mock {
            self.server
                .mock("POST", "/createCheck")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "check_id": 123,
                            "hash": "hash",
                            "asset": "TON",
                            "amount": "10.00",
                            "bot_check_url": "https://example.com/check",
                            "status": "active",
                            "created_at": "2021-01-01T00:00:00Z",
                            "activated_at": "2021-01-01T00:00:00Z",
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_checks_response_without_params(&mut self) -> Mock {
            self.server
                .mock("GET", "/getChecks")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "items": [
                                {
                                    "check_id": 123,
                                    "hash": "hash",
                                    "asset": "TON",
                                    "amount": "10.00",
                                    "bot_check_url": "https://example.com/check",
                                    "status": "active",
                                    "created_at": "2021-01-01T00:00:00Z",
                                    "activated_at": "2021-01-01T00:00:00Z",
                                }
                            ]
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_checks_response_with_check_ids(&mut self) -> Mock {
            self.server
                .mock("GET", "/getChecks")
                .match_body(json!({ "check_ids": "123" }).to_string().as_str())
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "items": [
                                {
                                    "check_id": 123,
                                    "hash": "hash",
                                    "asset": "TON",
                                    "amount": "10.00",
                                    "bot_check_url": "https://example.com/check",
                                    "status": "active",
                                    "created_at": "2021-01-01T00:00:00Z",
                                    "activated_at": "2021-01-01T00:00:00Z",
                                }
                            ]
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_checks_response_with_all_filters(&mut self) -> Mock {
            self.server
                .mock("GET", "/getChecks")
                .match_body(Matcher::JsonString(
                    json!({
                        "asset": "TON",
                        "check_ids": "1,2",
                        "status": "active",
                        "offset": 5,
                        "count": 10
                    })
                    .to_string(),
                ))
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "items": [
                                {
                                    "check_id": 321,
                                    "hash": "hash",
                                    "asset": "TON",
                                    "amount": "5.00",
                                    "bot_check_url": "https://example.com/check",
                                    "status": "active",
                                    "created_at": "2021-01-01T00:00:00Z",
                                    "activated_at": "2021-01-01T00:00:00Z",
                                }
                            ]
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_create_check_with_pin_response(&mut self) -> Mock {
            self.server
                .mock("POST", "/createCheck")
                .match_body(Matcher::JsonString(
                    json!({
                        "asset": "TON",
                        "amount": "5",
                        "pin_to_user_id": 99,
                        "pin_to_username": "alice"
                    })
                    .to_string(),
                ))
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "check_id": 321,
                            "hash": "hash",
                            "asset": "TON",
                            "amount": "5.00",
                            "bot_check_url": "https://example.com/check",
                            "status": "active",
                            "created_at": "2021-01-01T00:00:00Z",
                            "activated_at": "2021-01-01T00:00:00Z",
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_delete_check_response(&mut self) -> Mock {
            self.server
                .mock("DELETE", "/deleteCheck")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(json!({ "ok": true, "result": true }).to_string())
                .create()
        }
    }

    #[test]
    fn test_create_check() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_create_check_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .create_check()
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(10.0))
                .execute()
                .await
        });

        assert!(result.is_ok());

        let check = result.unwrap();
        assert_eq!(check.check_id, 123);
        assert_eq!(check.asset, CryptoCurrencyCode::Ton);
        assert_eq!(check.amount, dec!(10.0));
    }

    #[test]
    fn test_get_checks_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_checks_response_without_params();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_checks().execute().await });

        assert!(result.is_ok());

        let checks = result.unwrap();
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].check_id, 123);
    }

    #[test]
    fn test_get_checks_with_check_ids() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_checks_response_with_check_ids();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_checks().check_ids(vec![123]).execute().await });

        assert!(result.is_ok());

        let checks = result.unwrap();
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].check_id, 123);
    }

    #[test]
    fn test_get_checks_with_all_filters() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_checks_response_with_all_filters();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .get_checks()
                .asset(CryptoCurrencyCode::Ton)
                .check_ids(vec![1, 2])
                .status(CheckStatus::Active)
                .offset(5)
                .count(10)
                .execute()
                .await
        });

        assert!(result.is_ok());
        let checks = result.unwrap();
        assert_eq!(checks.len(), 1);
    }

    #[test]
    fn test_delete_check() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_delete_check_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.delete_check(123).execute().await });

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_create_check_with_pin_targets() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_create_check_with_pin_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .create_check()
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(5))
                .pin_to_user_id(99)
                .pin_to_username("alice")
                .execute()
                .await
        });

        assert!(result.is_ok());
        let check = result.unwrap();
        assert_eq!(check.check_id, 321);
    }

    #[test]
    fn test_get_checks_invalid_count() {
        let ctx = TestContext::new();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_checks().count(0).execute().await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                ..
            })
        ));
    }

    #[test]
    fn test_create_check_rejects_non_positive_amount() {
        let ctx = TestContext::new();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let builder = client.create_check().asset(CryptoCurrencyCode::Ton).amount(dec!(0));
        let result = builder.validate();

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                field,
                kind: ValidationErrorKind::Range,
                ..
            }) if field == Some("amount".to_string())
        ));
    }
}
