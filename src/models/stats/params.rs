use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct GetStatsParams {
    /// Optional. Date from which start calculating statistics in ISO 8601 format.
    /// Defaults is current date minus 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_at: Option<DateTime<Utc>>,

    /// Optional. Date to which end calculating statistics in ISO 8601 format.
    /// Defaults is current date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) end_at: Option<DateTime<Utc>>,
}
