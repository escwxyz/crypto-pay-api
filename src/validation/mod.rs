use crate::error::CryptoBotResult;
use crate::models::ExchangeRate;
use async_trait::async_trait;

pub trait FieldValidate {
    /// Validate every field of the model without context
    fn validate(&self) -> CryptoBotResult<()>;
}

#[async_trait]
pub trait ContextValidate {
    /// Validate field of the model with external context
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()>;
}

pub struct ValidationContext {
    pub exchange_rates: Vec<ExchangeRate>,
}

#[macro_export]
macro_rules! validate_dependency {
    ($condition:expr, $field:expr, $message:expr) => {
        if $condition {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Missing,
                message: $message.to_string(),
                field: Some($field.to_string()),
            });
        }
    };
}

mod amount;
mod currency;

pub use amount::*;
