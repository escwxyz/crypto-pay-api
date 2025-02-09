use crate::{
    error::CryptoBotResult,
    models::{APIMethod, Method, Transfer, TransferParams},
    validation::{ContextValidate, FieldValidate, ValidationContext},
    APIEndpoint, CryptoBot, GetTransfersParams, GetTransfersResponse,
};

use super::{ExchangeRateAPI, TransferAPI};

#[async_trait::async_trait]
impl TransferAPI for CryptoBot {
    /// Transfer cryptocurrency to a user
    ///
    /// # Arguments
    /// * `params` - Parameters for the transfer
    ///
    /// # Returns
    /// Returns Result with transfer information or CryptoBotError
    async fn transfer(&self, params: &TransferParams) -> CryptoBotResult<Transfer> {
        params.validate()?;

        let rates = self.get_exchange_rates().await?;

        let ctx = ValidationContext {
            exchange_rates: rates,
        };

        params.validate_with_context(&ctx).await?;

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
    /// * `asset` - Optional filter by asset
    /// * `transfer_ids` - Optional list of transfer IDs to filter
    /// * `offset` - Optional offset for pagination
    /// * `count` - Optional count of transfers to return
    ///
    /// # Returns
    /// Returns Result with vector of transfers or CryptoBotError
    async fn get_transfers(
        &self,
        params: Option<&GetTransfersParams>,
    ) -> CryptoBotResult<Vec<Transfer>> {
        if let Some(params) = params {
            params.validate()?;
        }

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
        models::{CryptoCurrencyCode, TransferStatus},
        utils::test_utils::TestContext,
    };

    use super::*;

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

    // ! Checked
    #[test]
    fn test_transfer() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_transfer_response();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);
        let params = TransferParams {
            user_id: 123456789,
            asset: CryptoCurrencyCode::Ton,
            amount: dec!(10.5),
            spend_id: "test_spend_id".to_string(),
            comment: Some("test_comment".to_string()),
            disable_send_notification: None,
        };

        let result = ctx.run(async { client.transfer(&params).await });

        println!("result:{:?}", result);

        assert!(result.is_ok());

        let transfer = result.unwrap();
        assert_eq!(transfer.transfer_id, 1);
        assert_eq!(transfer.user_id, 123456789);
        assert_eq!(transfer.asset, CryptoCurrencyCode::Ton);
        assert_eq!(transfer.amount, dec!(10.5));
        assert_eq!(transfer.status, TransferStatus::Completed);
    }

    // ! Checked
    #[test]
    fn test_get_transfers_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_transfers_response_without_params();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let params = GetTransfersParams {
            asset: Some(CryptoCurrencyCode::Ton),
            transfer_ids: Some(vec![1]),
            ..Default::default()
        };

        let result = ctx.run(async { client.get_transfers(Some(&params)).await });

        assert!(result.is_ok());
        let transfers = result.unwrap();
        assert_eq!(transfers.len(), 1);

        let transfer = &transfers[0];
        assert_eq!(transfer.transfer_id, 1);
        assert_eq!(transfer.asset, CryptoCurrencyCode::Ton);
        assert_eq!(transfer.status, TransferStatus::Completed);
    }

    // ! Checked
    #[test]
    fn test_get_transfers_with_transfer_ids() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_transfers_response_with_transfer_ids();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async {
            client
                .get_transfers(Some(&GetTransfersParams {
                    transfer_ids: Some(vec![1]),
                    ..Default::default()
                }))
                .await
        });

        assert!(result.is_ok());
        let transfers = result.unwrap();
        assert_eq!(transfers.len(), 1);
    }
}
