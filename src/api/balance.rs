use async_trait::async_trait;

use crate::{
    models::{APIMethod, Balance},
    APIEndpoint, CryptoBot, CryptoBotError, Method,
};

use super::BalanceAPI;

#[async_trait]
impl BalanceAPI for CryptoBot {
    /// Gets current balance for all supported currencies
    ///
    /// # Returns
    /// Returns Result with vector of [Balance] or CryptoBotError
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// let client = CryptoBot::new("test_token", None);
    /// let balances = client.get_balance().await;
    ///
    /// assert!(balances.is_ok());
    /// let balances = balances.unwrap();
    /// ```
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

    use crate::{models::CryptoCurrencyCode, test_utils::test_utils::TestContext};

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

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);
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
