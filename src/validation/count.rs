use crate::{error::CryptoBotError, error::CryptoBotResult, error::ValidationErrorKind};

pub fn validate_count(count: u16) -> CryptoBotResult<()> {
    if !(1..=1000).contains(&count) {
        return Err(CryptoBotError::ValidationError {
            kind: ValidationErrorKind::Range,
            message: "Count must be between 1 and 1000".to_string(),
            field: Some("count".to_string()),
        });
    }

    Ok(())
}
