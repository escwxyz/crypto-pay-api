use async_trait::async_trait;

use crate::{
    models::{APIMethod, ExchangeRate},
    APIEndpoint, CryptoBot, CryptoBotResult, Method,
};

use super::ExchangeRateAPI;

#[async_trait]
impl ExchangeRateAPI for CryptoBot {
    /// Gets exchange rates for supported currencies
    ///
    /// # Returns
    /// Returns Result with vector of exchange rates or CryptoBotError
    async fn get_exchange_rates(&self) -> CryptoBotResult<Vec<ExchangeRate>> {
        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::GetExchangeRates,
                method: Method::GET,
            },
            None::<()>.as_ref(),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use rust_decimal_macros::dec;
    use serde_json::json;

    use crate::{
        api::InvoiceAPI, error::ValidationErrorKind, models::CryptoCurrencyCode,
        test_utils::test_utils::TestContext, CheckAPI, CreateCheckParams, CreateInvoiceParams,
        CryptoBotError, CurrencyType, FiatCurrencyCode, TransferAPI, TransferParams,
    };

    use super::*;

    impl TestContext {
        pub fn mock_exchange_rates_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getExchangeRates")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": [
                        {
                            "is_valid": true,
                            "is_crypto": true,
                            "is_fiat": false,
                            "source": "TON",
                            "target": "USD",
                            "rate": "3.70824926"
                        },
                        {
                            "is_valid": true,
                            "is_crypto": true,
                            "is_fiat": false,
                            "source": "DOGE",
                            "target": "EUR",
                            "rate": "0.24000835"
                        },
                        {
                            "is_valid": true,
                            "is_crypto": true,
                            "is_fiat": false,
                            "source": "USDT",
                            "target": "RUB",
                            "rate": "96.92078586"
                        },
                        {
                            "is_valid": true,
                            "is_crypto": true,
                            "is_fiat": false,
                            "source": "TON",
                            "target": "EUR",
                            "rate": "3.59048268"
                        },
                        ]
                    })
                    .to_string(),
                )
                .create()
        }
    }

    // ! Checked
    #[test]
    fn test_get_exchange_rates() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async { client.get_exchange_rates().await });

        println!("result: {:?}", result);

        assert!(result.is_ok());

        let exchange_rates = result.unwrap();
        assert_eq!(exchange_rates.len(), 4);
        assert_eq!(exchange_rates[0].source, CryptoCurrencyCode::Ton);
        assert_eq!(exchange_rates[0].target, FiatCurrencyCode::Usd);
        assert_eq!(exchange_rates[0].rate, dec!(3.70824926));
    }

    // ! Checked
    /// Check if the amount is between 1 and 25000 USD
    #[test]
    fn test_validation_with_exchange_rates() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let params = CreateInvoiceParams {
            currency_type: Some(CurrencyType::Crypto),
            asset: Some(CryptoCurrencyCode::Ton),
            amount: dec!(10000.0),
            ..Default::default()
        };

        let result = ctx.run(async { client.create_invoice(&params).await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "amount"
        ));

        let params = CreateCheckParams {
            asset: CryptoCurrencyCode::Ton,
            amount: dec!(10000.0),
            pin_to_user_id: None,
            pin_to_username: None,
        };

        let result = ctx.run(async { client.create_check(&params).await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "amount"
        ));

        let params = TransferParams {
            asset: CryptoCurrencyCode::Ton,
            amount: dec!(10000.0),
            user_id: 123456789,
            spend_id: "test_spend_id".to_string(),
            comment: None,
            disable_send_notification: None,
        };

        let result = ctx.run(async { client.transfer(&params).await });

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "amount"
        ));
    }
}
