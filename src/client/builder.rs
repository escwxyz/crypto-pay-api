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

    pub fn api_token(mut self, api_token: impl Into<String>) -> Self {
        self.api_token = api_token.into();
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn headers(mut self, headers: Option<Vec<(HeaderName, HeaderValue)>>) -> Self {
        self.headers = headers;
        self
    }

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
