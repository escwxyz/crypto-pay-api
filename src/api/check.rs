use async_trait::async_trait;

use crate::{
    error::CryptoBotError,
    models::{APIMethod, Check, CreateCheckParams, GetChecksParams},
    validation::{ContextValidate, FieldValidate, ValidationContext},
    APIEndpoint, CryptoBot, DeleteCheckParams, GetChecksResponse, Method,
};

use super::{CheckAPI, ExchangeRateAPI};

#[async_trait]
impl CheckAPI for CryptoBot {
    /// Creates a new cryptocurrency check
    ///
    /// A check is a unique link that can be used once to transfer cryptocurrency.
    /// Anyone who opens the link first can activate the check and get the cryptocurrency.
    ///
    /// # Arguments
    /// * `params` - Parameters for creating a new check. See [`CreateCheckParams`] for details.
    ///
    /// # Returns
    /// * `Ok(Check)` - The created check
    /// * `Err(CryptoBotError)` - If validation fails or the request fails
    ///
    /// # Errors
    /// This method will return an error if:
    /// * The parameters are invalid (e.g., negative amount)
    /// * The currency is not supported
    /// * The API request fails
    /// * The exchange rate validation fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::new("YOUR_API_TOKEN", None);
    ///     
    ///     let params = CreateCheckParams::new()
    ///         .asset(CryptoCurrencyCode::Ton)
    ///         .amount(dec!(10.5));
    ///     
    ///     let check = client.create_check(&params).await?;
    ///     println!("Check created: {}", check.check_id);
    ///     
    ///     Ok(())
    /// }
    /// ```
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

    /// Deletes an existing cryptocurrency check
    ///
    /// Once deleted, the check link will become invalid and cannot be activated.
    ///
    /// # Arguments
    /// * `check_id` - The unique identifier of the check to delete
    ///
    /// # Returns
    /// * `Ok(true)` - If the check was successfully deleted
    /// * `Err(CryptoBotError)` - If the check doesn't exist or the request fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::new("YOUR_API_TOKEN", None);
    ///     
    ///     match client.delete_check(12345).await {
    ///         Ok(_) => println!("Check deleted successfully"),
    ///         Err(e) => eprintln!("Failed to delete check: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
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

    /// Gets a list of created cryptocurrency checks
    ///
    /// Retrieves all checks matching the specified filter parameters.
    /// If no parameters are provided, returns all checks.
    ///
    /// # Arguments
    /// * `params` - Optional filter parameters. See [`GetChecksParams`] for available filters.
    ///
    /// # Returns
    /// * `Ok(Vec<Check>)` - List of checks matching the filter criteria
    /// * `Err(CryptoBotError)` - If the parameters are invalid or the request fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::new("YOUR_API_TOKEN", None);
    ///     
    ///     // Get all checks
    ///     let all_checks = client.get_checks(None).await?;
    ///     
    ///     // Get checks with filters
    ///     let params = GetChecksParams::new()
    ///         .asset(CryptoCurrencyCode::Ton)
    ///         .status(CheckStatus::Active);
    ///     
    ///     let filtered_checks = client.get_checks(Some(&params)).await?;
    ///     
    ///     for check in filtered_checks {
    ///         println!("Check ID: {}, Amount: {} {}",
    ///             check.check_id,
    ///             check.amount,
    ///             check.asset
    ///         );
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Response Example
    /// ```json
    /// {
    ///   "items": [
    ///     {
    ///       "check_id": 12345,
    ///       "asset": "TON",
    ///       "amount": "10.5",
    ///       "status": "active",
    ///       "created_at": "2024-01-01T12:00:00.000Z",
    ///       "check_link": "https://t.me/CryptoBot?start=check_abc123",
    ///       "description": "Test check payment"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// # See Also
    /// * [Check](struct.Check.html) - The structure representing a check
    /// * [GetChecksParams](struct.GetChecksParams.html) - Available filter parameters
    /// * [CryptoBot API Documentation](https://help.crypt.bot/crypto-pay-api#getChecks)
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

    use crate::{utils::test_utils::TestContext, CryptoCurrencyCode};

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
