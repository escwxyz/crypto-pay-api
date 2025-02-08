use rust_decimal::Decimal;

use crate::error::{CryptoBotError, CryptoBotResult, ValidationErrorKind};
use crate::models::{CryptoCurrencyCode, FiatCurrencyCode};

use super::ValidationContext;

pub async fn validate_amount(
    amount: &Decimal,
    asset: &CryptoCurrencyCode,
    ctx: &ValidationContext,
) -> CryptoBotResult<()> {
    let usd_rate = ctx
        .exchange_rates
        .iter()
        .find(|rate| rate.source == *asset && rate.target == FiatCurrencyCode::Usd)
        .ok_or_else(|| CryptoBotError::ValidationError {
            kind: ValidationErrorKind::Missing,
            message: "exchange_rate_not_found".to_string(),
            field: Some("exchange_rate".to_string()),
        })?;

    let usd_value = amount * usd_rate.rate;

    if usd_value < Decimal::ONE || usd_value > Decimal::from(25000) {
        return Err(CryptoBotError::ValidationError {
            kind: ValidationErrorKind::Range,
            message: "Amount must be between 1 and 25000 USD".to_string(),
            field: Some("amount".to_string()),
        });
    }

    Ok(())
}
