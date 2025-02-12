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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ExchangeRate;
    use rust_decimal_macros::dec;

    fn create_test_context(rate: Decimal) -> ValidationContext {
        ValidationContext {
            exchange_rates: vec![ExchangeRate {
                source: CryptoCurrencyCode::Ton,
                target: FiatCurrencyCode::Usd,
                rate,
                is_valid: true,
                is_crypto: true,
                is_fiat: false,
            }],
        }
    }

    #[tokio::test]
    async fn test_validate_amount_valid() {
        let ctx = create_test_context(dec!(2.0)); // 1 TON = 2 USD

        // Test minimum valid amount (0.5 TON = 1 USD)
        assert!(validate_amount(&dec!(0.5), &CryptoCurrencyCode::Ton, &ctx)
            .await
            .is_ok());

        // Test maximum valid amount (12500 TON = 25000 USD)
        assert!(validate_amount(&dec!(12500), &CryptoCurrencyCode::Ton, &ctx)
            .await
            .is_ok());

        // Test middle range amount (50 TON = 100 USD)
        assert!(validate_amount(&dec!(50), &CryptoCurrencyCode::Ton, &ctx).await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_amount_too_small() {
        let ctx = create_test_context(dec!(2.0)); // 1 TON = 2 USD

        // Test amount that's too small (0.4 TON = 0.8 USD)
        let result = validate_amount(&dec!(0.4), &CryptoCurrencyCode::Ton, &ctx).await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message,
                field: Some(field),
            }) if message == "Amount must be between 1 and 25000 USD" && field == "amount"
        ));
    }

    #[tokio::test]
    async fn test_validate_amount_too_large() {
        let ctx = create_test_context(dec!(2.0)); // 1 TON = 2 USD

        // Test amount that's too large (12501 TON = 25002 USD)
        let result = validate_amount(&dec!(12501), &CryptoCurrencyCode::Ton, &ctx).await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message,
                field: Some(field),
            }) if message == "Amount must be between 1 and 25000 USD" && field == "amount"
        ));
    }

    #[tokio::test]
    async fn test_validate_amount_exchange_rate_not_found() {
        let ctx = create_test_context(dec!(2.0)); // Only has TON/USD rate

        // Test with BTC which has no exchange rate
        let result = validate_amount(&dec!(1), &CryptoCurrencyCode::Btc, &ctx).await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Missing,
                message,
                field: Some(field),
            }) if message == "exchange_rate_not_found" && field == "exchange_rate"
        ));
    }

    #[tokio::test]
    async fn test_validate_amount_edge_cases() {
        let ctx = create_test_context(dec!(2.0)); // 1 TON = 2 USD

        // Test exactly 1 USD
        assert!(validate_amount(&dec!(0.5), &CryptoCurrencyCode::Ton, &ctx)
            .await
            .is_ok());

        // Test exactly 25000 USD
        assert!(validate_amount(&dec!(12500), &CryptoCurrencyCode::Ton, &ctx)
            .await
            .is_ok());

        // Test slightly below 1 USD
        assert!(validate_amount(&dec!(0.499), &CryptoCurrencyCode::Ton, &ctx)
            .await
            .is_err());

        // Test slightly above 25000 USD
        assert!(validate_amount(&dec!(12500.01), &CryptoCurrencyCode::Ton, &ctx)
            .await
            .is_err());
    }
}
