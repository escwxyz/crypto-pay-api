use crate::{
    error::{CryptoBotError, CryptoBotResult},
    models::{APIMethod, ApiResponse, Method},
};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

const DEFAULT_API_URL: &str = "https://pay.crypt.bot/api";
const DEFAULT_TIMEOUT: u64 = 30;

#[derive(Debug, Clone)]
pub struct CryptoBot {
    pub api_token: String,
    client: Client,
    base_url: String,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
}

impl CryptoBot {
    /// Creates a new CryptoBot client instance
    ///
    /// # Arguments
    /// * `api_token` - The API token obtained from @CryptoBot
    /// * `headers` - Optional headers to be added to the request
    ///
    pub fn new(api_token: &str, headers: Option<Vec<(HeaderName, HeaderValue)>>) -> Self {
        Self::new_with_base_url(api_token, DEFAULT_API_URL, headers)
    }

    pub fn new_with_base_url(
        api_token: &str,
        base_url: &str,
        headers: Option<Vec<(HeaderName, HeaderValue)>>,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_token: api_token.to_string(),
            client,
            base_url: base_url.to_string(),
            headers,
        }
    }

    /// Makes a request to the CryptoBot API
    ///
    /// # Arguments
    /// * `method` - The method to call, must be one of the ApiMethod enum values
    /// * `params` - The parameters to pass to the method
    ///
    /// # Returns
    /// Returns Result with the response or CryptoBotError
    pub(crate) async fn make_request<T, R>(
        &self,
        method: &APIMethod,
        params: Option<&T>,
    ) -> CryptoBotResult<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, method.endpoint.as_str());
        println!("Making request to: {}", url);

        let mut request_headers = HeaderMap::new();
        println!("Constructing header");
        request_headers.insert(
            HeaderName::from_static("crypto-pay-api-token"),
            HeaderValue::from_str(&self.api_token)?,
        );

        if let Some(custom_headers) = &self.headers {
            println!("Adding custom headers");
            for (name, value) in custom_headers.iter() {
                request_headers.insert(name, value.clone());
            }
        }

        println!("Request builder");

        let mut request = match method.method {
            Method::POST => self.client.post(&url).headers(request_headers),
            Method::GET => self.client.get(&url).headers(request_headers),
            Method::DELETE => self.client.delete(&url).headers(request_headers),
        };

        if let Some(params) = params {
            println!("Adding params");
            request = request.json(params);
        }

        println!("Before response");

        let response = request.send().await?;

        println!("Got response");

        if !response.status().is_success() {
            return Err(CryptoBotError::HttpError(
                response.error_for_status().unwrap_err(),
            ));
        }

        println!("Response status: {}", response.status());
        let text = response.text().await?;
        println!("Response body: {}", text);

        let api_response: ApiResponse<R> =
            serde_json::from_str(&text).map_err(|e| CryptoBotError::ApiError {
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
}
