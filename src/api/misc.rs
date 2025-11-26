use chrono::{DateTime, Utc};

use crate::{
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    models::{APIEndpoint, APIMethod, AppStats, Currency, GetMeResponse, GetStatsParams, Method},
};
use async_trait::async_trait;

use super::MiscAPI;

pub struct GetMeBuilder<'a> {
    client: &'a CryptoBot,
}

impl<'a> GetMeBuilder<'a> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self { client }
    }

    /// Executes the request to get application information
    pub async fn execute(self) -> CryptoBotResult<GetMeResponse> {
        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetMe,
                    method: Method::GET,
                },
                None::<()>.as_ref(),
            )
            .await
    }
}

pub struct GetCurrenciesBuilder<'a> {
    client: &'a CryptoBot,
}

impl<'a> GetCurrenciesBuilder<'a> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self { client }
    }

    /// Executes the request to get supported currencies
    pub async fn execute(self) -> CryptoBotResult<Vec<Currency>> {
        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetCurrencies,
                    method: Method::GET,
                },
                None::<()>.as_ref(),
            )
            .await
    }
}

pub struct GetStatsBuilder<'a> {
    client: &'a CryptoBot,
    params: GetStatsParams,
}

impl<'a> GetStatsBuilder<'a> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self {
            client,
            params: GetStatsParams::default(),
        }
    }

    /// Set the start date for the statistics.
    /// Optional. Defaults is current date minus 24 hours.
    pub fn start_at(mut self, start_at: DateTime<Utc>) -> Self {
        self.params.start_at = Some(start_at);
        self
    }

    /// Set the end date for the statistics.
    /// Optional. Defaults is current date.
    pub fn end_at(mut self, end_at: DateTime<Utc>) -> Self {
        self.params.end_at = Some(end_at);
        self
    }

    /// Executes the request to get application statistics
    pub async fn execute(self) -> CryptoBotResult<AppStats> {
        let now = Utc::now();

        if let Some(start) = self.params.start_at {
            if start > now {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "start_at cannot be in the future".to_string(),
                    field: Some("start_at".to_string()),
                });
            }
        }

        if let (Some(start), Some(end)) = (self.params.start_at, self.params.end_at) {
            if end < start {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "end_at cannot be earlier than start_at".to_string(),
                    field: Some("end_at".to_string()),
                });
            }
        }

        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetStats,
                    method: Method::GET,
                },
                Some(&self.params),
            )
            .await
    }
}

#[async_trait]
impl MiscAPI for CryptoBot {
    /// Gets basic information about your application
    ///
    /// Retrieves information about your application, including app ID, name,
    /// and payment processing bot username.
    ///
    /// # Returns
    /// * `GetMeBuilder` - A builder to execute the request
    fn get_me(&self) -> GetMeBuilder<'_> {
        GetMeBuilder::new(self)
    }

    /// Gets a list of all supported cryptocurrencies
    ///
    /// Returns information about all cryptocurrencies supported by CryptoBot,
    /// including both crypto and fiat currencies.
    ///
    /// # Returns
    /// * `GetCurrenciesBuilder` - A builder to execute the request
    fn get_currencies(&self) -> GetCurrenciesBuilder<'_> {
        GetCurrenciesBuilder::new(self)
    }

    /// Gets application statistics for a specified time period
    ///
    /// Retrieves statistics about your application's usage, including
    /// transaction volumes, number of invoices, and user counts.
    ///
    /// # Returns
    /// * `GetStatsBuilder` - A builder to construct the filter parameters
    fn get_stats(&self) -> GetStatsBuilder<'_> {
        GetStatsBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use mockito::Mock;
    use rust_decimal::Decimal;
    use serde_json::json;

    use crate::{
        api::MiscAPI,
        client::CryptoBot,
        models::{CryptoCurrencyCode, CurrencyCode},
        utils::test_utils::TestContext,
    };

    impl TestContext {
        pub fn mock_get_me_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getMe")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "app_id": 28692,
                            "name": "Stated Seaslug App",
                            "payment_processing_bot_username": "CryptoTestnetBot"
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_currencies_response(&mut self) -> Mock {
            println!("Setting up mock response");
            self.server
                .mock("GET", "/getCurrencies")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": [
                            {
                                "is_blockchain": false,
                                "is_stablecoin": true,
                                "is_fiat": false,
                                "name": "Tether",
                                "code": "USDT",
                                "url": "https://tether.to/",
                                "decimals": 18
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Toncoin",
                                "code": "TON",
                                "url": "https://ton.org/",
                                "decimals": 9
                            },
                            // ... other currencies omitted for brevity in test ...
                        ]
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_stats_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getStats")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "volume": 0,
                            "conversion": 0,
                            "unique_users_count": 0,
                            "created_invoice_count": 0,
                            "paid_invoice_count": 0,
                            "start_at": "2025-02-07T10:55:17.438Z",
                            "end_at": "2025-02-08T10:55:17.438Z"
                        }
                    })
                    .to_string(),
                )
                .create()
        }
    }

    #[test]
    fn test_get_me() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_me_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_me().execute().await });

        println!("Result: {:?}", result);

        assert!(result.is_ok());
        let me = result.unwrap();
        assert_eq!(me.app_id, 28692);
        assert_eq!(me.name, "Stated Seaslug App");
        assert_eq!(me.payment_processing_bot_username, "CryptoTestnetBot");
        assert_eq!(me.webhook_endpoint, None);
    }

    #[test]
    fn test_get_currencies() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_currencies_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_currencies().execute().await });

        assert!(result.is_ok());
        let currencies = result.unwrap();

        assert_eq!(currencies.len(), 2); // Mocked only 2
        assert_eq!(currencies[0].code, CurrencyCode::Crypto(CryptoCurrencyCode::Usdt));
        assert_eq!(currencies[1].code, CurrencyCode::Crypto(CryptoCurrencyCode::Ton));
    }

    #[test]
    fn test_get_stats_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_stats_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_stats().execute().await });

        println!("result: {:?}", result);

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.volume, Decimal::from(0));
        assert_eq!(stats.conversion, Decimal::from(0));
    }

    #[test]
    fn test_get_stats_with_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_stats_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .get_stats()
                .start_at(Utc::now() - Duration::days(7))
                .end_at(Utc::now())
                .execute()
                .await
        });

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.volume, Decimal::from(0));
        assert_eq!(stats.conversion, Decimal::from(0));
    }
}
