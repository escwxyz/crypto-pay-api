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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_count_valid() {
        assert!(validate_count(1).is_ok());
        assert!(validate_count(100).is_ok());
        assert!(validate_count(500).is_ok());
        assert!(validate_count(1000).is_ok());
    }

    #[test]
    fn test_validate_count_too_small() {
        let result = validate_count(0);
        assert!(result.is_err());
        match result {
            Err(CryptoBotError::ValidationError { kind, message, field }) => {
                assert_eq!(kind, ValidationErrorKind::Range);
                assert_eq!(message, "Count must be between 1 and 1000");
                assert_eq!(field, Some("count".to_string()));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_validate_count_too_large() {
        let result = validate_count(1001);
        assert!(result.is_err());
        match result {
            Err(CryptoBotError::ValidationError { kind, message, field }) => {
                assert_eq!(kind, ValidationErrorKind::Range);
                assert_eq!(message, "Count must be between 1 and 1000");
                assert_eq!(field, Some("count".to_string()));
            }
            _ => panic!("Expected ValidationError"),
        }
    }
}
