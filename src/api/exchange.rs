use async_trait::async_trait;

use crate::{
    client::CryptoBot,
    error::CryptoBotResult,
    models::{APIEndpoint, APIMethod, ExchangeRate, Method},
};

use super::ExchangeRateAPI;

#[async_trait]
impl ExchangeRateAPI for CryptoBot {
    /// Gets current exchange rates for all supported cryptocurrencies
    ///
    /// This method returns exchange rates between supported cryptocurrencies and target currencies.
    /// Exchange rates are updated regularly by CryptoBot.
    ///
    /// # Returns
    /// * `Ok(Vec<ExchangeRate>)` - A list of current exchange rates
    /// * `Err(CryptoBotError)` - If the request fails
    ///
    /// # Exchange Rate Pairs
    /// Exchange rates are provided for various pairs:
    /// * Cryptocurrency to fiat (e.g., TON/USD, BTC/EUR)
    /// * Cryptocurrency to cryptocurrency (e.g., TON/BTC, ETH/BTC)
    /// * Test currencies are also included in testnet mode
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///     
    ///     let rates = client.get_exchange_rates().await?;
    ///     
    ///     for rate in rates {
    ///         println!("Exchange Rate: {} {} = {}",
    ///             rate.source,
    ///             rate.target,
    ///             rate.rate,
    ///         );
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [ExchangeRate](struct.ExchangeRate.html) - The structure representing an exchange rate
    async fn get_exchange_rates(&self) -> CryptoBotResult<Vec<ExchangeRate>> {
        #[cfg(test)]
        if let Some(rates) = &self.test_rates {
            return Ok(rates.clone());
        }

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
        models::{CryptoCurrencyCode, FiatCurrencyCode},
        utils::test_utils::TestContext,
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

    #[test]
    fn test_get_exchange_rates() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_exchange_rates().await });

        println!("result: {:?}", result);

        assert!(result.is_ok());

        let exchange_rates = result.unwrap();
        assert_eq!(exchange_rates.len(), 4);
        assert_eq!(exchange_rates[0].source, CryptoCurrencyCode::Ton);
        assert_eq!(exchange_rates[0].target, FiatCurrencyCode::Usd);
        assert_eq!(exchange_rates[0].rate, dec!(3.70824926));
    }
}
