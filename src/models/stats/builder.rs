use chrono::{DateTime, Utc};

use crate::{
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    validation::FieldValidate,
};

use super::GetStatsParams;

#[derive(Debug, Default)]
pub struct GetStatsParamsBuilder {
    start_at: Option<DateTime<Utc>>,
    end_at: Option<DateTime<Utc>>,
}

impl GetStatsParamsBuilder {
    /// Create a new `GetStatsParamsBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the start date for the statistics.
    /// Optional. Defaults is current date minus 24 hours.
    pub fn start_at(mut self, start_at: DateTime<Utc>) -> Self {
        self.start_at = Some(start_at);
        self
    }

    /// Set the end date for the statistics.
    /// Optional. Defaults is current date.
    pub fn end_at(mut self, end_at: DateTime<Utc>) -> Self {
        self.end_at = Some(end_at);
        self
    }
}

impl FieldValidate for GetStatsParamsBuilder {
    fn validate(&self) -> CryptoBotResult<()> {
        let now = Utc::now();

        if let Some(start) = self.start_at {
            if start > now {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "start_at cannot be in the future".to_string(),
                    field: Some("start_at".to_string()),
                });
            }
        }

        if let (Some(start), Some(end)) = (self.start_at, self.end_at) {
            if end < start {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "end_at cannot be earlier than start_at".to_string(),
                    field: Some("end_at".to_string()),
                });
            }
        }

        Ok(())
    }
}

impl GetStatsParamsBuilder {
    pub fn build(self) -> CryptoBotResult<GetStatsParams> {
        self.validate()?;
        Ok(GetStatsParams {
            start_at: self.start_at,
            end_at: self.end_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stats_params_builder() {
        let params = GetStatsParamsBuilder::new().start_at(Utc::now()).build();

        assert!(params.is_ok());
    }

    #[test]
    fn test_get_stats_params_builder_invalid_start_at() {
        let params = GetStatsParamsBuilder::new()
            .start_at(Utc::now() + chrono::Duration::days(1))
            .build();

        assert!(matches!(
            params,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "start_at"
        ));
    }
    #[test]
    fn test_get_stats_params_builder_invalid_end_at() {
        let params = GetStatsParamsBuilder::new()
            .start_at(Utc::now())
            .end_at(Utc::now() - chrono::Duration::days(1))
            .build();

        assert!(matches!(
            params,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "end_at"
        ));
    }

    #[test]
    fn test_get_stats_params_builder_invalid_end_at_and_start_at() {
        let params = GetStatsParamsBuilder::new()
            .start_at(Utc::now() - chrono::Duration::days(2))
            .end_at(Utc::now() - chrono::Duration::days(3))
            .build();

        assert!(matches!(
            params,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "end_at"
        ));
    }
}
