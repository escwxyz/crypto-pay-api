use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    error::ValidationErrorKind, validation::FieldValidate, CryptoBotError, CryptoBotResult,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppStats {
    /// Total volume of paid invoices in USD.
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_decimal_from_number")]
    pub volume: Decimal,

    /// Conversion of all created invoices.
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_decimal_from_number")]
    pub conversion: Decimal,

    /// The unique number of users who have paid the invoice.
    pub unique_users_count: u64,

    /// Total created invoice count.
    pub created_invoice_count: u64,

    /// Total paid invoice count.
    pub paid_invoice_count: u64,

    /// The date on which the statistics calculation was started in ISO 8601 format.
    pub start_at: DateTime<Utc>,

    /// The date on which the statistics calculation was ended in ISO 8601 format.
    pub end_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GetStatsParams {
    /// Optional. Date from which start calculating statistics in ISO 8601 format.
    /// Defaults is current date minus 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<DateTime<Utc>>,

    /// Optional. Date to which end calculating statistics in ISO 8601 format.
    /// Defaults is current date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<DateTime<Utc>>,
}

impl FieldValidate for GetStatsParams {
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

            // if end - start > Duration::days(365) {
            //     return Err(CryptoBotError::ValidationError {
            //         kind: ValidationErrorKind::Range,
            //         message: "Time range cannot exceed 365 days".to_string(),
            //         field: Some("start_at".to_string()),
            //     });
            // }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ! Checked
    #[test]
    fn test_get_stats_params_default() {
        let params = GetStatsParams::default();
        assert_eq!(params.start_at, None);
        assert_eq!(params.end_at, None);
    }

    // ! Checked
    #[test]
    fn test_get_stats_params_validate() {
        let params = GetStatsParams {
            start_at: Some(Utc::now() + chrono::Duration::days(1)),
            end_at: Some(Utc::now()),
        };

        let result = params.validate();

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "start_at"
        ));
    }
}
