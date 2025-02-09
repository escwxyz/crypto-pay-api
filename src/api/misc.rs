use crate::{
    error::CryptoBotResult,
    models::{APIMethod, AppStats, Currency, GetMeResponse, GetStatsParams},
    validation::FieldValidate,
    APIEndpoint, CryptoBot, Method,
};
use async_trait::async_trait;

use super::MiscAPI;

#[async_trait]
impl MiscAPI for CryptoBot {
    /// Gets basic information about an application
    ///
    /// # Returns
    /// Returns basic information about an application or CryptoBotError
    async fn get_me(&self) -> CryptoBotResult<GetMeResponse> {
        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::GetMe,
                method: Method::GET,
            },
            None::<()>.as_ref(),
        )
        .await
    }

    /// Gets supported currencies
    ///
    /// # Returns
    /// Returns Result with vector of supported currencies or CryptoBotError
    async fn get_currencies(&self) -> CryptoBotResult<Vec<Currency>> {
        let method = APIMethod {
            endpoint: APIEndpoint::GetCurrencies,
            method: Method::GET,
        };

        self.make_request(&method, None::<()>.as_ref()).await
    }

    /// Gets app statistics
    ///
    /// # Arguments
    /// * `params` - Parameters for getting statistics
    ///
    /// # Returns
    /// Returns Result with app statistics or CryptoBotError
    async fn get_stats(&self, params: Option<&GetStatsParams>) -> CryptoBotResult<AppStats> {
        if let Some(params) = params {
            params.validate()?;
        }

        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::GetStats,
                method: Method::GET,
            },
            params,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use rust_decimal::Decimal;
    use serde_json::json;

    use crate::{
        api::MiscAPI, utils::test_utils::TestContext, CryptoBot, CryptoCurrencyCode, CurrencyCode,
        GetStatsParams,
    };

    impl TestContext {
        pub fn mock_get_me_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getMe")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "app_id": 28692,
                            "name": "Stated Seaslug App",
                            "payment_processing_bot_username": "CryptoTestnetBot"
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_currencies_response(&mut self) -> Mock {
            println!("Setting up mock response");
            self.server
                .mock("GET", "/getCurrencies")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": [
                            {
                                "is_blockchain": false,
                                "is_stablecoin": true,
                                "is_fiat": false,
                                "name": "Tether",
                                "code": "USDT",
                                "url": "https://tether.to/",
                                "decimals": 18
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Toncoin",
                                "code": "TON",
                                "url": "https://ton.org/",
                                "decimals": 9
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Bitcoin",
                                "code": "BTC",
                                "url": "https://bitcoin.org/",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Dogecoin",
                                "code": "DOGE",
                                "url": "https://dogecoin.org/",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Litecoin",
                                "code": "LTC",
                                "url": "https://litecoin.org/",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Ethereum",
                                "code": "ETH",
                                "url": "https://ethereum.org/",
                                "decimals": 18
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Binance Coin",
                                "code": "BNB",
                                "url": "https://binance.org/",
                                "decimals": 18
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "TRON",
                                "code": "TRX",
                                "url": "https://tron.network/",
                                "decimals": 6
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": true,
                                "is_fiat": false,
                                "name": "USD Coin",
                                "code": "USDC",
                                "url": "https://www.centre.io/usdc",
                                "decimals": 18
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": true,
                                "is_fiat": false,
                                "name": "TON Jetton",
                                "code": "JET",
                                "url": "https://ton.org",
                                "decimals": 9
                            },
                            {
                                "is_blockchain": true,
                                "is_stablecoin": false,
                                "is_fiat": false,
                                "name": "Crypto Bot",
                                "code": "SEND",
                                "url": "https://send.tg/",
                                "decimals": 9
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Russian ruble",
                                "code": "RUB",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "United States Dollar",
                                "code": "USD",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Euro",
                                "code": "EUR",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Belarusian ruble",
                                "code": "BYN",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Ukrainian hryvnia",
                                "code": "UAH",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Pound sterling",
                                "code": "GBP",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Chinese yuan renminbi",
                                "code": "CNY",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Kazakhstani tenge",
                                "code": "KZT",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Uzbekistani som",
                                "code": "UZS",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Georgian lari",
                                "code": "GEL",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Turkish lira",
                                "code": "TRY",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Armenian dram",
                                "code": "AMD",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Thai baht",
                                "code": "THB",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Indian rupee",
                                "code": "INR",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Brazilian real",
                                "code": "BRL",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Indonesian rupiah",
                                "code": "IDR",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Azerbaijani manat",
                                "code": "AZN",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "United Arab Emirates dirham",
                                "code": "AED",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Polish zloty",
                                "code": "PLN",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Israeli new shekel",
                                "code": "ILS",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Kyrgystani som",
                                "code": "KGS",
                                "decimals": 8
                            },
                            {
                                "is_blockchain": false,
                                "is_stablecoin": false,
                                "is_fiat": true,
                                "name": "Tajikistani somoni",
                                "code": "TJS",
                                "decimals": 8
                            }
                        ]
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_stats_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getStats")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "volume": 0,
                            "conversion": 0,
                            "unique_users_count": 0,
                            "created_invoice_count": 0,
                            "paid_invoice_count": 0,
                            "start_at": "2025-02-07T10:55:17.438Z",
                            "end_at": "2025-02-08T10:55:17.438Z"
                        }
                    })
                    .to_string(),
                )
                .create()
        }
    }

    // ! Checked
    #[test]
    fn test_get_me() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_me_response();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);
        let result = ctx.run(async { client.get_me().await });

        println!("Result: {:?}", result);

        assert!(result.is_ok());
        let me = result.unwrap();
        assert_eq!(me.app_id, 28692);
        assert_eq!(me.name, "Stated Seaslug App");
        assert_eq!(me.payment_processing_bot_username, "CryptoTestnetBot");
        assert_eq!(me.webhook_endpoint, None);
    }

    // ! Checked
    #[test]
    fn test_get_currencies() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_currencies_response();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async { client.get_currencies().await });

        assert!(result.is_ok());
        let currencies = result.unwrap();

        assert_eq!(currencies.len(), 33);
        assert_eq!(
            currencies[0].code,
            CurrencyCode::Crypto(CryptoCurrencyCode::Usdt)
        );
        assert_eq!(
            currencies[1].code,
            CurrencyCode::Crypto(CryptoCurrencyCode::Ton)
        );
    }

    // ! Checked
    #[test]
    fn test_get_stats() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_stats_response();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async { client.get_stats(None::<&GetStatsParams>).await });

        println!("result: {:?}", result);

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.volume, Decimal::from(0));
        assert_eq!(stats.conversion, Decimal::from(0));
    }
}
