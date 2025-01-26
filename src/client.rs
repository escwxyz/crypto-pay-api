use crate::{
    error::CryptoBotError,
    model::{ApiResponse, Currency, GetMeResponse},
    webhook::WebhookUpdate,
};

use axum::http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client as HttpClient;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

const DEFAULT_API_URL: &str = "https://pay.crypt.bot/api";
const DEFAULT_TIMEOUT: u64 = 30;

#[derive(Debug, Clone)]
pub struct CryptoBot {
    api_token: String,
    client: HttpClient,
    base_url: String,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
}

#[derive(Debug)]
pub enum ParameterFormat {
    Json,
    FormUrlEncoded,
    MultipartFormData,
    QueryString,
}

impl CryptoBot {
    /// Creates a new CryptoBot client instance
    ///
    /// # Arguments
    /// * `api_token` - The API token obtained from @CryptoBot
    ///
    /// # Example
    /// ```
    /// use crypto_bot_api::CryptoBot;
    ///
    /// let client = CryptoBot::new("1234:AAA...AAA");
    /// ```
    pub fn new(api_token: &str, headers: Option<Vec<(HeaderName, HeaderValue)>>) -> Self {
        Self::new_with_base_url(api_token, DEFAULT_API_URL, headers)
    }

    pub fn new_with_base_url(
        api_token: &str,
        base_url: &str,
        headers: Option<Vec<(HeaderName, HeaderValue)>>,
    ) -> Self {
        let client = HttpClient::builder()
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

    /// Gets information about the bot
    ///
    /// # Returns
    /// Returns Result with bot information or CryptoBotError
    pub async fn get_me(&self) -> Result<GetMeResponse, CryptoBotError> {
        self.make_request("getMe", None::<()>.as_ref()).await
    }

    /// Gets supported currencies
    ///
    /// # Returns
    /// Returns Result with vector of supported currencies or CryptoBotError
    pub async fn get_currencies(&self) -> Result<Vec<Currency>, CryptoBotError> {
        self.make_request("getCurrencies", None::<()>.as_ref())
            .await
    }

    /// Verify webhook request authenticity
    ///
    /// # Arguments
    /// * `token` - The token from the webhook request header
    ///
    /// # Returns
    /// Returns true if the webhook request is authentic
    pub fn verify_webhook(&self, token: &str) -> bool {
        token == self.api_token
    }

    /// Parse webhook update from JSON
    ///
    /// # Arguments
    /// * `json` - The JSON string from the webhook request body
    ///
    /// # Returns
    /// Returns Result with WebhookUpdate or CryptoBotError
    pub fn parse_webhook_update(json: &str) -> Result<WebhookUpdate, CryptoBotError> {
        serde_json::from_str(json).map_err(|e| CryptoBotError::WebhookError(e.to_string()))
    }

    pub(crate) async fn make_request<T, R>(
        &self,
        method: &str,
        params: Option<&T>,
    ) -> Result<R, CryptoBotError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, method);

        let mut request_headers = HeaderMap::new();
        request_headers.insert(
            HeaderName::from_static("crypto-pay-api-token"),
            HeaderValue::from_str(&self.api_token)?,
        );
        request_headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );

        if let Some(custom_headers) = &self.headers {
            for (name, value) in custom_headers.iter() {
                request_headers.insert(name, value.clone());
            }
        }

        let mut request = self.client.post(&url).headers(request_headers);

        if let Some(params) = params {
            request = request.json(params);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(CryptoBotError::HttpError(
                response.error_for_status().unwrap_err(),
            ));
        }

        let result = response.json::<ApiResponse<R>>().await?;

        match result {
            ApiResponse {
                ok: true,
                result: Some(data),
                ..
            } => Ok(data),
            ApiResponse {
                ok: false,
                error: Some(error),
                error_code: Some(code),
                ..
            } => Err(CryptoBotError::ApiError { code, name: error }),
            _ => Err(CryptoBotError::ApiError {
                code: -1,
                name: "Unknown error".to_string(),
            }),
        }
    }
    // TODO: Add tests for this
    // async fn make_request_with_format<T, R>(
    //     &self,
    //     method: &str,
    //     params: Option<&T>,
    //     format: ParameterFormat,
    // ) -> Result<R, CryptoBotError>
    // where
    //     T: Serialize + ?Sized,
    //     R: for<'de> Deserialize<'de>,
    // {
    //     let url = format!("{}/{}", self.base_url.to_string(), method);

    //     let mut request = self
    //         .client
    //         .post(&url)
    //         .header("Crypto-Pay-API-Token", &self.api_token);

    //     if let Some(params) = params {
    //         request = match format {
    //             ParameterFormat::Json => request
    //                 .header("Content-Type", "application/json")
    //                 .json(params),
    //             ParameterFormat::FormUrlEncoded => request
    //                 .header("Content-Type", "application/x-www-form-urlencoded")
    //                 .form(params),
    //             ParameterFormat::QueryString => request.query(params),
    //             ParameterFormat::MultipartFormData => {
    //                 todo!("Multipart form data not yet implemented")
    //             }
    //         };
    //     }

    //     let response = request.send().await?;

    //     if !response.status().is_success() {
    //         return Err(CryptoBotError::HttpError(
    //             response.error_for_status().unwrap_err(),
    //         ));
    //     }

    //     let result = response.json::<ApiResponse<R>>().await?;

    //     match result {
    //         ApiResponse {
    //             ok: true,
    //             result: Some(data),
    //             ..
    //         } => Ok(data),
    //         ApiResponse {
    //             ok: false,
    //             error: Some(error),
    //             error_code: Some(code),
    //             ..
    //         } => Err(CryptoBotError::ApiError { code, name: error }),
    //         _ => Err(CryptoBotError::ApiError {
    //             code: -1,
    //             name: "Unknown error".to_string(),
    //         }),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use serde_json::json;

    use super::*;
    use crate::{
        invoice::{CreateInvoiceParams, InvoiceStatus},
        model::{CryptoCurrencyCode, CurrencyType},
        test_utils::test_utils::TestContext,
        traits::InvoiceClient,
    };

    impl TestContext {
        pub fn mock_currencies_response(&mut self) -> Mock {
            self.server
                .mock("POST", "/getCurrencies")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": [{
                            "is_blockchain": true,
                            "is_stablecoin": false,
                            "is_fiat": false,
                            "name": "Toncoin",
                            "code": "TON",
                            "url": "https://ton.org",
                            "decimals": 9
                        }]
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_error_response(
            &mut self,
            method: &str,
            error_code: i32,
            error_message: &str,
        ) -> Mock {
            self.server
                .mock("POST", method)
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": false,
                        "error": error_message,
                        "error_code": error_code
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_confirm_payment_response(&mut self) -> Mock {
            self.server
                .mock("POST", "/confirmPayment")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "invoice_id": 1,
                            "status": "paid",
                            "hash": "TEST_HASH",
                            "asset": "TON",
                            "amount": "10.5",
                            "pay_url": "https://pay.crypt.bot/TEST_HASH",
                            "description": "Test invoice",
                            "created_at": "2024-03-14T12:00:00Z",
                            "paid_at": "2024-03-14T12:01:00Z",
                            "allow_comments": true,
                            "allow_anonymous": true,
                            "paid_anonymously": false,
                            "comment": null,
                            "hidden_message": null,
                            "paid_btn_name": null,
                            "paid_btn_url": null
                        }
                    })
                    .to_string(),
                )
                .create()
        }
    }

    #[test]
    fn test_get_currencies() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_currencies_response();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);
        let result = ctx.run(async { client.get_currencies().await });

        assert!(result.is_ok());
        let currencies = result.unwrap();
        assert_eq!(currencies.len(), 1);
        assert_eq!(currencies[0].code, CryptoCurrencyCode::Ton);
    }

    #[test]
    fn test_error_handling() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_error_response("/createInvoice", 404, "Invoice not found");

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);
        let params = CreateInvoiceParams {
            currency_type: Some(CurrencyType::Crypto),
            asset: Some(CryptoCurrencyCode::Ton),
            amount: "10.5".to_string(),
            description: None,
            hidden_message: None,
            paid_btn_name: None,
            paid_btn_url: None,
            payload: None,
            allow_comments: None,
            allow_anonymous: None,
            expires_in: None,
            fiat: None,
            accept_asset: Some(vec![CryptoCurrencyCode::Ton]),
        };

        let result = ctx.run(async { client.create_invoice(&params).await });
        assert!(result.is_err());

        match result {
            Err(CryptoBotError::ApiError { code, name }) => {
                assert_eq!(code, 404);
                assert_eq!(name, "Invoice not found");
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_confirm_payment() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_confirm_payment_response();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);
        let paid_at = chrono::Utc::now();

        let result = ctx.run(async { client.confirm_paid_invoice(1, paid_at).await });
        assert!(result.is_ok());

        let invoice = result.unwrap();
        assert_eq!(invoice.status, InvoiceStatus::Paid);
        assert_eq!(invoice.invoice_id, 1);
        assert_eq!(invoice.asset, Some(CryptoCurrencyCode::Ton));
    }
}
