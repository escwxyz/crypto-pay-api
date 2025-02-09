use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoBotError {
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("HTTP client error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error: {code} - {message}")]
    ApiError {
        code: i32,
        message: String,
        details: Option<serde_json::Value>,
    },

    #[error("Validation error: {kind} - {message}")]
    ValidationError {
        kind: ValidationErrorKind,
        message: String,
        field: Option<String>,
    },

    #[error("Webhook error: {kind} - {message}")]
    WebhookError {
        kind: WebhookErrorKind,
        message: String,
    },

    #[error("No result returned from API")]
    NoResult,
}

#[derive(Debug)]
pub enum ValidationErrorKind {
    Format,
    Range,
    Currency,
    Missing,
    Invalid,
}

#[derive(Debug)]
pub enum WebhookErrorKind {
    InvalidSignature,
    InvalidPayload,
    DeserializationError,
    Expired,
}

impl std::fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for WebhookErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type CryptoBotResult<T> = std::result::Result<T, CryptoBotError>;
