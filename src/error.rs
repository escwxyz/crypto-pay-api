use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoBotError {
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(InvalidHeaderValue),
    #[error("HTTP client error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("API error: {code} - {name}")]
    ApiError { code: i32, name: String },
    #[error("Webhook error: {0}")]
    WebhookError(String),
}

impl From<InvalidHeaderValue> for CryptoBotError {
    fn from(error: InvalidHeaderValue) -> Self {
        CryptoBotError::InvalidHeaderValue(error)
    }
}
