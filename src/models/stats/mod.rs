mod builder;
mod params;

pub use builder::*;
pub use params::*;

use crate::utils::deserialize_decimal;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppStats {
    /// Total volume of paid invoices in USD.
    #[serde(deserialize_with = "deserialize_decimal")]
    pub volume: Decimal,

    /// Conversion of all created invoices.
    #[serde(deserialize_with = "deserialize_decimal")]
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
