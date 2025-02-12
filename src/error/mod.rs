use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoBotError {
    #[error("Invalid header name: {0}")]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),

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
    WebhookError { kind: WebhookErrorKind, message: String },

    #[error("No result returned from API")]
    NoResult,
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_kind_display() {
        let test_cases = vec![
            (ValidationErrorKind::Range, "Range"),
            (ValidationErrorKind::Format, "Format"),
            (ValidationErrorKind::Missing, "Missing"),
            (ValidationErrorKind::Invalid, "Invalid"),
            (ValidationErrorKind::Currency, "Currency"),
        ];

        for (kind, expected) in test_cases {
            assert_eq!(kind.to_string(), expected);
        }
    }

    #[test]
    fn test_webhook_error_kind_display() {
        let test_cases = vec![
            (WebhookErrorKind::InvalidSignature, "InvalidSignature"),
            (WebhookErrorKind::InvalidPayload, "InvalidPayload"),
            (WebhookErrorKind::DeserializationError, "DeserializationError"),
            (WebhookErrorKind::Expired, "Expired"),
        ];

        for (kind, expected) in test_cases {
            assert_eq!(kind.to_string(), expected);
        }
    }

    #[test]
    fn test_validation_error_formatting() {
        let error = CryptoBotError::ValidationError {
            kind: ValidationErrorKind::Range,
            message: "Value out of range".to_string(),
            field: Some("amount".to_string()),
        };

        assert_eq!(error.to_string(), "Validation error: Range - Value out of range");
    }

    #[test]
    fn test_webhook_error_formatting() {
        let error = CryptoBotError::WebhookError {
            kind: WebhookErrorKind::InvalidSignature,
            message: "Invalid signature".to_string(),
        };

        assert_eq!(error.to_string(), "Webhook error: InvalidSignature - Invalid signature");
    }
}
