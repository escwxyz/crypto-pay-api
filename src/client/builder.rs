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
