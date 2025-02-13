mod builder;

use std::str::FromStr;

use crate::{
    error::{CryptoBotError, CryptoBotResult},
    models::{APIMethod, ApiResponse, Method},
};

#[cfg(test)]
use crate::models::ExchangeRate;

use builder::{ClientBuilder, NoAPIToken};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{de::DeserializeOwned, Serialize};

pub const DEFAULT_API_URL: &str = "https://pay.crypt.bot/api";
pub const DEFAULT_TIMEOUT: u64 = 30;
pub const DEFAULT_WEBHOOK_EXPIRATION_TIME: u64 = 600;

#[derive(Debug)]
pub struct CryptoBot {
    pub(crate) api_token: String,
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) headers: Option<Vec<(HeaderName, HeaderValue)>>,
    #[cfg(test)]
    pub(crate) test_rates: Option<Vec<ExchangeRate>>,
}

impl CryptoBot {
    /// Returns a new builder for creating a customized CryptoBot client
    ///
    /// The builder pattern allows you to customize all aspects of the client,
    /// including timeout, base URL and headers settings.
    ///
    /// # Available Settings
    /// * `api_token` - Required, the API token from [@CryptoBot](https://t.me/CryptoBot)
    /// * `base_url` - Optional, defaults to "https://pay.crypt.bot/api"
    /// * `timeout` - Optional, defaults to 30 seconds
    /// * `headers` - Optional, custom headers for all requests
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder()
    ///         .api_token("YOUR_API_TOKEN")
    ///         .base_url("https://testnet-pay.crypt.bot/api")  // Use testnet
    ///         .timeout(Duration::from_secs(60))               // 60 second timeout
    ///     .headers(vec![(
    ///         HeaderName::from_static("x-custom-header"),
    ///         HeaderValue::from_static("custom_value")
    ///     )])
    ///     .build()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [`ClientBuilder`](struct.ClientBuilder.html) - The builder type
    pub fn builder() -> ClientBuilder<NoAPIToken> {
        ClientBuilder::new()
    }

    /// Makes a request to the CryptoBot API
    ///
    /// # Arguments
    /// * `method` - The method to call, must be one of the ApiMethod enum values
    /// * `params` - The parameters to pass to the method
    ///
    /// # Returns
    /// * `Ok(R)` - The response from the API
    /// * `Err(CryptoBotError)` - If the request fails or the response is not valid
    pub(crate) async fn make_request<T, R>(&self, method: &APIMethod, params: Option<&T>) -> CryptoBotResult<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, method.endpoint.as_str());

        let mut request_headers = HeaderMap::new();

        let token_header = HeaderName::from_str("Crypto-Pay-Api-Token")?;

        request_headers.insert(token_header, HeaderValue::from_str(&self.api_token)?);

        if let Some(custom_headers) = &self.headers {
            for (name, value) in custom_headers.iter() {
                request_headers.insert(name, value.clone());
            }
        }

        let mut request = match method.method {
            Method::POST => self.client.post(&url).headers(request_headers),
            Method::GET => self.client.get(&url).headers(request_headers),
            Method::DELETE => self.client.delete(&url).headers(request_headers),
        };

        if let Some(params) = params {
            request = request.json(params);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(CryptoBotError::HttpError(response.error_for_status().unwrap_err()));
        }

        let text = response.text().await?;

        let api_response: ApiResponse<R> = serde_json::from_str(&text).map_err(|e| CryptoBotError::ApiError {
            code: -1,
            message: "Failed to parse API response".to_string(),
            details: Some(serde_json::json!({ "error": e.to_string() })),
        })?;

        if !api_response.ok {
            return Err(CryptoBotError::ApiError {
                code: api_response.error_code.unwrap_or(0),
                message: api_response.error.unwrap_or_default(),
                details: None,
            });
        }

        api_response.result.ok_or(CryptoBotError::NoResult)
    }

    #[cfg(test)]
    pub fn test_client() -> Self {
        use crate::utils::test_utils::TestContext;

        Self {
            api_token: "test_token".to_string(),
            client: reqwest::Client::new(),
            base_url: "http://test.example.com".to_string(),
            headers: None,
            test_rates: Some(TestContext::mock_exchange_rates()),
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use serde_json::json;

    use crate::{
        api::BalanceAPI,
        models::{APIEndpoint, Balance},
        utils::test_utils::TestContext,
    };

    use super::*;
    impl TestContext {
        pub fn mock_malformed_json_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getBalance")
                .with_header("content-type", "application/json")
                .with_body("invalid json{")
                .create()
        }
    }

    #[test]
    fn test_malformed_json_response() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_malformed_json_response();

        let client = CryptoBot::builder()
            .api_token("test")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ApiError {
                code: -1,
                message,
                details: Some(details)
            }) if message == "Failed to parse API response"
            && details.get("error").is_some()
        ));
    }

    #[test]
    fn test_invalid_response_structure() {
        let mut ctx = TestContext::new();

        let _m = ctx
            .server
            .mock("GET", "/getBalance")
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": true,
                    "result": "not_an_array"
                })
                .to_string(),
            )
            .create();

        let client = CryptoBot::builder()
            .api_token("test")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ApiError {
                code: -1,
                message,
                details: Some(details)
            }) if message == "Failed to parse API response"
                && details.get("error").is_some()
        ));
    }

    #[test]
    fn test_empty_response() {
        let mut ctx = TestContext::new();

        // Mock empty response
        let _m = ctx
            .server
            .mock("GET", "/getBalance")
            .with_header("content-type", "application/json")
            .with_body("")
            .create();

        let client = CryptoBot::builder()
            .api_token("test")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ApiError {
                code: -1,
                message,
                details: Some(details)
            }) if message == "Failed to parse API response"
                && details.get("error").is_some()
        ));
    }

    #[test]
    fn test_invalid_api_token_header() {
        let client = CryptoBot {
            api_token: "invalid\u{0000}token".to_string(),
            client: reqwest::Client::new(),
            base_url: "http://test.example.com".to_string(),
            headers: None,
            #[cfg(test)]
            test_rates: None,
        };

        let method = APIMethod {
            endpoint: APIEndpoint::GetBalance,
            method: Method::GET,
        };
        let ctx = TestContext::new();

        let result = ctx.run(async { client.make_request::<(), Vec<Balance>>(&method, None).await });

        assert!(matches!(result, Err(CryptoBotError::InvalidHeaderValue(_))));
    }

    #[test]
    fn test_api_error_response() {
        let mut ctx = TestContext::new();

        // Mock API error response with error code and message
        let _m = ctx
            .server
            .mock("GET", "/getBalance")
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": false,
                    "error": "Test error message",
                    "error_code": 123
                })
                .to_string(),
            )
            .create();

        let client = CryptoBot::builder()
            .api_token("test")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ApiError {
                code,
                message,
                details,
            }) if code == 123
                && message == "Test error message"
                && details.is_none()
        ));
    }

    #[test]
    fn test_api_error_response_missing_fields() {
        let mut ctx = TestContext::new();

        // Mock API error response without error code and message
        let _m = ctx
            .server
            .mock("GET", "/getBalance")
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": false
                })
                .to_string(),
            )
            .create();

        let client = CryptoBot::builder()
            .api_token("test")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ApiError {
                code,
                message,
                details,
            }) if code == 0
                && message.is_empty()
                && details.is_none()
        ));
    }

    #[test]
    fn test_api_error_response_partial_fields() {
        let mut ctx = TestContext::new();

        // Mock API error response with only error message
        let _m = ctx
            .server
            .mock("GET", "/getBalance")
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "ok": false,
                    "error": "Test error message"
                })
                .to_string(),
            )
            .create();

        let client = CryptoBot::builder()
            .api_token("test")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ApiError {
                code,
                message,
                details,
            }) if code == 0
                && message == "Test error message"
                && details.is_none()
        ));
    }

    #[test]
    fn test_http_error_response() {
        let mut ctx = TestContext::new();
        let _m = ctx.server.mock("GET", "/getBalance").with_status(404).create();

        let client = CryptoBot::builder()
            .api_token("test")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(matches!(result, Err(CryptoBotError::HttpError(_))));
    }
}
