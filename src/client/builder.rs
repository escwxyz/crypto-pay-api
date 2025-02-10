use reqwest::header::{HeaderName, HeaderValue};
use std::time::Duration;

use super::{CryptoBot, DEFAULT_API_URL, DEFAULT_TIMEOUT};

pub struct ClientBuilder {
    api_token: String,
    base_url: String,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
    timeout: Duration,
}

impl std::default::Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            api_token: String::new(),
            base_url: DEFAULT_API_URL.to_string(),
            headers: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        }
    }

    /// Set the API token
    ///
    /// # Arguments
    /// * `api_token` - The API token
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// let client = CryptoBot::builder()
    ///     .api_token("YOUR_API_TOKEN")
    ///     .build();
    /// ```
    pub fn api_token(mut self, api_token: impl Into<String>) -> Self {
        self.api_token = api_token.into();
        self
    }

    /// Set the base URL
    ///
    /// # Arguments
    /// * `base_url` - The base URL
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// let client = CryptoBot::builder()
    ///     .base_url("https://pay.crypt.bot/api") // default is https://pay.crypt.bot/api, you can use the testnet API endpoint for testing
    ///     .build();
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set the headers
    ///
    /// # Arguments
    /// * `headers` - The headers
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// let client = CryptoBot::builder()
    ///     .api_token("test_token")
    ///     .headers(Some(vec![(
    ///         HeaderName::from_static("x-custom-header"),
    ///         HeaderValue::from_static("custom_value")
    ///     )]))
    ///     .build();
    /// ```
    pub fn headers(mut self, headers: Option<Vec<(HeaderName, HeaderValue)>>) -> Self {
        self.headers = headers;
        self
    }

    /// Set the timeout, default is 30 seconds
    ///
    /// # Arguments
    /// * `timeout` - The timeout
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use crypto_pay_api::prelude::*;
    ///
    /// let client = CryptoBot::builder()
    ///     .timeout(Duration::from_secs(10))
    ///     .build();
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build the CryptoBot client
    pub fn build(self) -> CryptoBot {
        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .expect("Failed to create HTTP client");

        CryptoBot {
            api_token: self.api_token,
            client,
            base_url: self.base_url,
            headers: self.headers,
        }
    }
}
