use async_trait::async_trait;

use crate::{
    models::{APIMethod, Check, CreateCheckParams, GetChecksParams},
    validation::{ContextValidate, FieldValidate, ValidationContext},
    APIEndpoint, CryptoBot, CryptoBotError, DeleteCheckParams, GetChecksResponse, Method,
};

use super::{CheckAPI, ExchangeRateAPI};

#[async_trait]
impl CheckAPI for CryptoBot {
    /// Creates a new check
    ///
    /// # Arguments
    /// * `params` - Parameters for creating a new check
    ///
    /// # Returns
    /// Returns Result with check or CryptoBotError
    async fn create_check(&self, params: &CreateCheckParams) -> Result<Check, CryptoBotError> {
        params.validate()?;

        let rates = self.get_exchange_rates().await?;

        let ctx = ValidationContext {
            exchange_rates: rates,
        };

        params.validate_with_context(&ctx).await?;

        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::CreateCheck,
                method: Method::POST,
            },
            Some(params),
        )
        .await
    }

    /// Deletes a check
    ///
    /// # Arguments
    /// * `check_id` - ID of the check to delete
    ///
    /// # Returns
    /// Returns Result with true on success or CryptoBotError
    async fn delete_check(&self, check_id: u64) -> Result<bool, CryptoBotError> {
        let params = DeleteCheckParams { check_id };

        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::DeleteCheck,
                method: Method::DELETE,
            },
            Some(&params),
        )
        .await
    }

    /// Gets checks by specified parameters
    ///
    /// # Arguments
    /// * `params` - Parameters for filtering checks
    ///
    /// # Returns
    /// Returns Result with vector of checks or CryptoBotError
    async fn get_checks(
        &self,
        params: Option<&GetChecksParams>,
    ) -> Result<Vec<Check>, CryptoBotError> {
        if let Some(params) = params {
            params.validate()?;
        }

        let response: GetChecksResponse = self
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetChecks,
                    method: Method::GET,
                },
                params,
            )
            .await?;

        Ok(response.items)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use rust_decimal_macros::dec;
    use serde_json::json;

    use crate::{test_utils::test_utils::TestContext, CryptoCurrencyCode};

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

        pub fn mock_delete_check_response(&mut self) -> Mock {
            self.server
                .mock("DELETE", "/deleteCheck")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(json!({ "ok": true, "result": true }).to_string())
                .create()
        }
    }

    // ! Checked
    #[test]
    fn test_create_check() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_create_check_response();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async {
            client
                .create_check(&CreateCheckParams {
                    asset: CryptoCurrencyCode::Ton,
                    amount: dec!(10.0),
                    pin_to_user_id: None,
                    pin_to_username: None,
                })
                .await
        });

        assert!(result.is_ok());

        let check = result.unwrap();
        assert_eq!(check.check_id, 123);
        assert_eq!(check.asset, CryptoCurrencyCode::Ton);
        assert_eq!(check.amount, dec!(10.0));
    }

    // ! Checked
    #[test]
    fn test_get_checks_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_checks_response_without_params();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async { client.get_checks(None).await });

        assert!(result.is_ok());

        let checks = result.unwrap();
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].check_id, 123);
    }

    // ! Checked
    #[test]
    fn test_get_checks_with_check_ids() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_checks_response_with_check_ids();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async {
            client
                .get_checks(Some(&GetChecksParams {
                    check_ids: Some(vec![123]),
                    ..Default::default()
                }))
                .await
        });

        assert!(result.is_ok());

        let checks = result.unwrap();
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].check_id, 123);
    }

    // ! Checked
    #[test]
    fn test_delete_check() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_delete_check_response();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async { client.delete_check(123).await });

        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
