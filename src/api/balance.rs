use async_trait::async_trait;

use crate::{
    client::CryptoBot,
    error::CryptoBotError,
    models::{APIEndpoint, APIMethod, Balance, Method},
};

use super::BalanceAPI;

#[async_trait]
impl BalanceAPI for CryptoBot {
    /// Gets current balance for all supported cryptocurrencies in your CryptoBot wallet
    ///
    /// This method returns the current balance for all cryptocurrencies that are
    /// available in your CryptoBot wallet, including both crypto and test currencies.
    ///
    /// # Returns
    /// * `Ok(Vec<Balance>)` - A vector of balances for each currency
    /// * `Err(CryptoBotError)` - If the request fails
    ///
    /// # Errors
    /// This method will return an error if:
    /// * The API request fails
    /// * The response cannot be parsed
    /// * The API token is invalid
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///     
    ///     let balances = client.get_balance().await?;
    ///     
    ///     for balance in balances {
    ///         println!("Currency: {}", balance.currency_code);
    ///         println!("Available: {}", balance.available);
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [Balance](struct.Balance.html) - The structure representing a currency balance
    /// * [CryptoBot API Documentation](https://help.crypt.bot/crypto-pay-api#getBalance)
    async fn get_balance(&self) -> Result<Vec<Balance>, CryptoBotError> {
        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::GetBalance,
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

    use crate::{models::CryptoCurrencyCode, utils::test_utils::TestContext};

    use super::*;

    impl TestContext {
        pub fn mock_balance_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getBalance")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": [
                        {
                            "currency_code": "TON",
                            "available": "100.5",
                            "onhold": "0.0"
                        },
                        {
                            "currency_code": "SEND",
                            "available": "10.5",
                            "onhold": "0.0"
                        }
                        ]
                    })
                    .to_string(),
                )
                .create()
        }
    }

    #[test]
    fn test_get_balance() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_balance_response();

        let client = CryptoBot::builder()
            .api_token("api_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_balance().await });

        assert!(result.is_ok());
        let balances = result.unwrap();
        assert_eq!(balances.len(), 2);
        assert_eq!(balances[0].currency_code, CryptoCurrencyCode::Ton);
        assert_eq!(balances[0].available, dec!(100.5));
        assert_eq!(balances[1].currency_code, CryptoCurrencyCode::Send);
        assert_eq!(balances[1].available, dec!(10.5));
    }
}
