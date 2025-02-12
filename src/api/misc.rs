use crate::{
    client::CryptoBot,
    error::CryptoBotResult,
    models::{APIEndpoint, APIMethod, AppStats, Currency, GetMeResponse, GetStatsParams, Method},
};
use async_trait::async_trait;

use super::MiscAPI;

#[async_trait]
impl MiscAPI for CryptoBot {
    /// Gets basic information about your application
    ///
    /// Retrieves information about your application, including app ID, name,
    /// and payment processing bot username.
    ///
    /// # Returns
    /// * `Ok(GetMeResponse)` - Basic information about your application
    /// * `Err(CryptoBotError)` - If the request fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///     
    ///     let app_info = client.get_me().await?;
    ///     println!("App ID: {}", app_info.app_id);
    ///     println!("App Name: {}", app_info.name);
    ///     println!("Bot Username: {}", app_info.payment_processing_bot_username);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [GetMeResponse](struct.GetMeResponse.html) - The structure representing the response
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

    /// Gets a list of all supported cryptocurrencies
    ///
    /// Returns information about all cryptocurrencies supported by CryptoBot,
    /// including both crypto and fiat currencies.
    ///
    /// # Returns
    /// * `Ok(Vec<Currency>)` - List of supported currencies with their properties
    /// * `Err(CryptoBotError)` - If the request fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///     
    ///     let currencies = client.get_currencies().await?;
    ///     
    ///     for currency in currencies {
    ///         println!("Currency: {}", currency.name);
    ///         println!("Type: {}", if currency.is_blockchain { "Crypto" }
    ///             else if currency.is_fiat { "Fiat" }
    ///             else { "Stablecoin" });
    ///         println!("Decimals: {}", currency.decimals);
    ///         if let Some(url) = currency.url {
    ///             println!("Website: {}", url);
    ///         }
    ///         println!("---");
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [Currency](struct.Currency.html) - The structure representing a currency
    async fn get_currencies(&self) -> CryptoBotResult<Vec<Currency>> {
        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::GetCurrencies,
                method: Method::GET,
            },
            None::<()>.as_ref(),
        )
        .await
    }

    /// Gets application statistics for a specified time period
    ///
    /// Retrieves statistics about your application's usage, including
    /// transaction volumes, number of invoices, and user counts.
    ///
    /// # Arguments
    /// * `params` - Optional parameters to filter statistics by date range.
    ///             See [`GetStatsParams`] for available options.
    ///
    /// # Returns
    /// * `Ok(AppStats)` - Application statistics for the specified period
    /// * `Err(CryptoBotError)` - If the parameters are invalid or the request fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    /// use chrono::{Utc, Duration};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///     
    ///     // Get stats for the last 7 days
    ///     let end_date = Utc::now();
    ///     let start_date = end_date - Duration::days(7);
    ///     
    ///     let params = GetStatsParamsBuilder::new()
    ///         .start_at(start_date)
    ///         .end_at(end_date)
    ///         .build()
    ///         .unwrap();
    ///     
    ///     let stats = client.get_stats(Some(&params)).await?;
    ///     
    ///     println!("Statistics for the last 7 days:");
    ///     println!("Total volume: {}", stats.volume);
    ///     println!("Number of invoices created: {}", stats.created_invoice_count);
    ///     println!("Number of paid invoices: {}", stats.paid_invoice_count);
    ///     println!("Unique users: {}", stats.unique_users_count);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [AppStats](struct.AppStats.html) - The structure representing application statistics
    /// * [GetStatsParams](struct.GetStatsParams.html) - Parameters for filtering statistics
    async fn get_stats(&self, params: Option<&GetStatsParams>) -> CryptoBotResult<AppStats> {
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
        api::MiscAPI,
        client::CryptoBot,
        models::{CryptoCurrencyCode, CurrencyCode, GetStatsParams},
        utils::test_utils::TestContext,
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
        // TODO add more data
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

    #[test]
    fn test_get_me() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_me_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_me().await });

        println!("Result: {:?}", result);

        assert!(result.is_ok());
        let me = result.unwrap();
        assert_eq!(me.app_id, 28692);
        assert_eq!(me.name, "Stated Seaslug App");
        assert_eq!(me.payment_processing_bot_username, "CryptoTestnetBot");
        assert_eq!(me.webhook_endpoint, None);
    }

    #[test]
    fn test_get_currencies() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_currencies_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_currencies().await });

        assert!(result.is_ok());
        let currencies = result.unwrap();

        assert_eq!(currencies.len(), 33);
        assert_eq!(currencies[0].code, CurrencyCode::Crypto(CryptoCurrencyCode::Usdt));
        assert_eq!(currencies[1].code, CurrencyCode::Crypto(CryptoCurrencyCode::Ton));
    }

    #[test]
    fn test_get_stats_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_stats_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_stats(None::<&GetStatsParams>).await });

        println!("result: {:?}", result);

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.volume, Decimal::from(0));
        assert_eq!(stats.conversion, Decimal::from(0));
    }

    // #[test]
    // fn test_get_stats_with_params() {
    //     let mut ctx = TestContext::new();
    //     let _m = ctx.mock_get_stats_response();

    //     let client = CryptoBot::builder()
    //         .api_token("test_token")
    //         .base_url(ctx.server.url())
    //         .build()
    //         .unwrap();

    //     let params = GetStatsParamsBuilder::new()
    //         .start_at(Utc::now() - Duration::days(7))
    //         .end_at(Utc::now())
    //         .build()
    //         .unwrap();

    //     let result = ctx.run(async { client.get_stats(Some(&params)).await });

    //     assert!(result.is_ok());
    //     let stats = result.unwrap();
    //     assert_eq!(stats.volume, Decimal::from(0));
    //     assert_eq!(stats.conversion, Decimal::from(0));
    // }
}
