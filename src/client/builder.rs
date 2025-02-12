use reqwest::header::{HeaderName, HeaderValue};
use std::time::Duration;

use crate::error::CryptoBotResult;

use super::{CryptoBot, DEFAULT_API_URL, DEFAULT_TIMEOUT};

pub struct NoAPIToken;

pub struct ClientBuilder<T> {
    api_token: T,
    base_url: String,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
    timeout: Duration,
}

impl<T> ClientBuilder<T> {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn headers(mut self, headers: Vec<(HeaderName, HeaderValue)>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl ClientBuilder<NoAPIToken> {
    pub fn new() -> Self {
        Self {
            api_token: NoAPIToken,
            base_url: DEFAULT_API_URL.to_string(),
            headers: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        }
    }

    pub fn api_token(self, api_token: impl Into<String>) -> ClientBuilder<String> {
        ClientBuilder {
            api_token: api_token.into(),
            base_url: self.base_url,
            headers: self.headers,
            timeout: self.timeout,
        }
    }
}

impl Default for ClientBuilder<NoAPIToken> {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder<String> {
    pub fn build(self) -> CryptoBotResult<CryptoBot> {
        let client = reqwest::Client::builder().timeout(self.timeout).build()?;
        Ok(CryptoBot {
            api_token: self.api_token,
            client,
            base_url: self.base_url,
            headers: self.headers,
            #[cfg(test)]
            test_rates: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use reqwest::header::HeaderName;
    use std::str::FromStr;

    use crate::{api::ExchangeRateAPI, utils::test_utils::TestContext};

    use super::*;

    #[test]
    fn test_builder_default_config() {
        let builder = ClientBuilder::new();
        let client = builder.api_token("test").build().unwrap();
        assert_eq!(client.base_url, DEFAULT_API_URL);
    }

    #[test]
    fn test_builder_custom_config() {
        let builder = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .base_url("https://test.com");

        let client = builder.api_token("test").build().unwrap();

        assert_eq!(client.base_url, "https://test.com".to_string());
    }

    #[test]
    fn test_builder_custom_headers() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();

        let builder = ClientBuilder::new()
            .headers(vec![(
                HeaderName::from_str("X-Custom-Header").unwrap(),
                HeaderValue::from_static("test"),
            )])
            .timeout(Duration::from_secs(30))
            .base_url(ctx.server.url());

        let client = builder.api_token("test").build().unwrap();

        // headers are only set when making requests
        let _ = ctx.run(async { client.get_exchange_rates().await });

        assert!(client
            .headers
            .as_ref()
            .map(|headers| headers.contains(&(
                HeaderName::from_str("X-Custom-Header").unwrap(),
                HeaderValue::from_static("test"),
            )))
            .unwrap_or(false));
    }
}
