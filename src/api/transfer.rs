use crate::{
    client::CryptoBot,
    error::CryptoBotResult,
    models::{APIEndpoint, APIMethod, GetTransfersParams, GetTransfersResponse, Method, Transfer, TransferParams},
};

use super::TransferAPI;

#[async_trait::async_trait]
impl TransferAPI for CryptoBot {
    /// Transfer cryptocurrency to a user
    ///
    /// # Arguments
    /// * `params` - Parameters for the transfer, including:
    ///   - `user_id`: Telegram user ID
    ///   - `asset`: Cryptocurrency code (e.g., "TON", "BTC")
    ///   - `amount`: Amount to transfer
    ///   - `spend_id`: Optional unique ID to ensure idempotence
    ///   - `comment`: Optional comment for transfer
    ///   - `disable_send_notification`: Optional flag to disable notification
    ///
    /// # Returns
    /// Returns Result containing transfer information or CryptoBotError
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> CryptoBotResult<()> {
    ///     let client = CryptoBot::builder().api_token("your_token").build()?;
    ///
    ///     let params = TransferParamsBuilder::new()
    ///         .user_id(123456789)
    ///         .asset(CryptoCurrencyCode::Ton)
    ///         .amount(dec!(10.5))
    ///         .spend_id("unique_id")
    ///         .comment("Payment for services")
    ///         .build(&client)
    ///         .await?;
    ///
    ///     let transfer = client.transfer(&params).await?;
    ///     
    ///     println!("Transfer ID: {}", transfer.transfer_id);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See also
    /// * [GetTransfersParamsBuilder](crate::models::GetTransfersParamsBuilder)
    /// * [TransferParamsBuilder](crate::models::TransferParamsBuilder)
    async fn transfer(&self, params: &TransferParams) -> CryptoBotResult<Transfer> {
        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::Transfer,
                method: Method::POST,
            },
            Some(params),
        )
        .await
    }
    /// Get transfers history
    ///
    /// # Arguments
    /// * `params` - Optional parameters to filter transfers:
    ///   - `asset`: Filter by cryptocurrency code
    ///   - `transfer_ids`: List of specific transfer IDs to retrieve
    ///   - `spend_id`: Unique ID to filter transfers by spend ID
    ///   - `offset`: Number of records to skip (for pagination)
    ///   - `count`: Maximum number of records to return (1-1000)
    ///
    /// # Returns
    /// Returns Result containing a vector of transfers or CryptoBotError
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> CryptoBotResult<()> {
    ///     let client = CryptoBot::builder().api_token("your_token").build()?;
    ///
    ///     // Get all transfers
    ///     let all_transfers = client.get_transfers(None).await?;
    ///
    ///     // Get filtered transfers
    ///     let params = GetTransfersParamsBuilder::new()
    ///         .asset(CryptoCurrencyCode::Ton)
    ///         .transfer_ids(vec![1, 2, 3])
    ///         .count(10)
    ///         .build()?;
    ///
    ///     let filtered_transfers = client.get_transfers(Some(&params)).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See also
    /// * [GetTransfersParamsBuilder](crate::models::GetTransfersParamsBuilder)
    async fn get_transfers(&self, params: Option<&GetTransfersParams>) -> CryptoBotResult<Vec<Transfer>> {
        let response: GetTransfersResponse = self
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetTransfers,
                    method: Method::GET,
                },
                params,
            )
            .await?;

        Ok(response.items)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use rust_decimal_macros::dec;
    use serde_json::json;

    use crate::{
        api::TransferAPI,
        client::CryptoBot,
        models::{CryptoCurrencyCode, GetTransfersParamsBuilder, TransferParamsBuilder, TransferStatus},
        utils::test_utils::TestContext,
    };

    impl TestContext {
        pub fn mock_transfer_response(&mut self) -> Mock {
            self.server
                .mock("POST", "/transfer")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "transfer_id": 1,
                            "user_id": 123456789,
                            "asset": "TON",
                            "amount": "10.5",
                            "status": "completed",
                            "completed_at": "2024-03-14T12:00:00Z",
                            "comment": "test_comment",
                            "spend_id": "test_spend_id",
                            "disable_send_notification": false,
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_transfers_response_without_params(&mut self) -> Mock {
            self.server
                .mock("GET", "/getTransfers")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "items": [{
                                "transfer_id": 1,
                                "user_id": 123456789,
                                "asset": "TON",
                                "amount": "10.5",
                                "status": "completed",
                                "completed_at": "2024-03-14T12:00:00Z",
                                "comment": "test_comment",
                                "spend_id": "test_spend_id",
                                "disable_send_notification": false,
                            }]
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_transfers_response_with_transfer_ids(&mut self) -> Mock {
            self.server
                .mock("GET", "/getTransfers")
                .match_body(json!({ "transfer_ids": "1" }).to_string().as_str())
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "items": [
                                {
                                    "transfer_id": 1,
                                    "user_id": 123456789,
                                    "asset": "TON",
                                    "amount": "10.5",
                                    "status": "completed",
                                    "completed_at": "2024-03-14T12:00:00Z",
                                    "comment": "test_comment",
                                    "spend_id": "test_spend_id",
                                    "disable_send_notification": false,
                                }
                            ]
                        }
                    })
                    .to_string(),
                )
                .create()
        }
    }

    #[test]
    fn test_transfer() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_transfer_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            let params = TransferParamsBuilder::new()
                .user_id(123456789)
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(10.5))
                .spend_id("test_spend_id".to_string())
                .comment("test_comment".to_string())
                .build(&client)
                .await
                .unwrap();
            client.transfer(&params).await
        });

        println!("result:{:?}", result);

        assert!(result.is_ok());

        let transfer = result.unwrap();
        assert_eq!(transfer.transfer_id, 1);
        assert_eq!(transfer.user_id, 123456789);
        assert_eq!(transfer.asset, CryptoCurrencyCode::Ton);
        assert_eq!(transfer.amount, dec!(10.5));
        assert_eq!(transfer.status, TransferStatus::Completed);
    }

    #[test]
    fn test_get_transfers_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_transfers_response_without_params();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let params = GetTransfersParamsBuilder::new()
            .asset(CryptoCurrencyCode::Ton)
            .transfer_ids(vec![1])
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_transfers(Some(&params)).await });

        assert!(result.is_ok());
        let transfers = result.unwrap();
        assert_eq!(transfers.len(), 1);

        let transfer = &transfers[0];
        assert_eq!(transfer.transfer_id, 1);
        assert_eq!(transfer.asset, CryptoCurrencyCode::Ton);
        assert_eq!(transfer.status, TransferStatus::Completed);
    }

    #[test]
    fn test_get_transfers_with_transfer_ids() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_transfers_response_with_transfer_ids();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let params = GetTransfersParamsBuilder::new().transfer_ids(vec![1]).build().unwrap();

        let result = ctx.run(async { client.get_transfers(Some(&params)).await });

        assert!(result.is_ok());
        let transfers = result.unwrap();
        assert_eq!(transfers.len(), 1);
    }
}
