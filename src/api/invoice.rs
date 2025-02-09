use async_trait::async_trait;

use crate::{
    models::{CreateInvoiceParams, GetInvoicesParams, Invoice},
    validation::{ContextValidate, FieldValidate},
    APIEndpoint, APIMethod, CryptoBotError, CryptoBotResult, DeleteInvoiceParams,
    GetInvoicesResponse, Method,
};

use super::{ExchangeRateAPI, InvoiceAPI};
use crate::validation::ValidationContext;

#[async_trait]
impl InvoiceAPI for crate::CryptoBot {
    /// Creates a new invoice
    ///
    /// # Arguments
    /// * `params` - Parameters for creating the invoice
    ///
    /// # Returns
    /// Returns Result with created invoice or CryptoBotError
    ///
    /// # Examples
    /// ```rust
    /// use crypto_pay_api::prelude::*;
    ///
    /// let bot = CryptoBot::new("test_token");
    /// let params = CreateInvoiceParams {
    ///     currency_type: Some(CurrencyType::Crypto),
    ///     asset: Some(CryptoCurrencyCode::Ton),
    ///     amount: dec!(10.5),
    ///     description: Some("Test invoice".to_string()),
    ///     ..Default::default()
    /// };
    /// let invoice = bot.create_invoice(&params).await?;
    ///
    /// assert_eq!(invoice.amount, dec!(10.5));
    /// assert_eq!(invoice.asset, Some(CryptoCurrencyCode::Ton));
    /// assert_eq!(invoice.description, Some("Test invoice".to_string()));
    /// ```
    ///
    async fn create_invoice(
        &self,
        params: &CreateInvoiceParams,
    ) -> Result<Invoice, CryptoBotError> {
        params.validate()?;

        let rates = self.get_exchange_rates().await?;

        let ctx = ValidationContext {
            exchange_rates: rates,
        };

        params.validate_with_context(&ctx).await?;

        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::CreateInvoice,
                method: Method::POST,
            },
            Some(params),
        )
        .await
    }

    /// Deletes an invoice
    ///
    /// # Arguments
    /// * `invoice_id` - ID of the invoice to delete
    ///
    /// # Returns
    /// Returns Result with true on success or CryptoBotError
    ///
    /// # Examples
    /// ```rust
    /// use crypto_pay_api::prelude::*;
    ///
    /// let bot = CryptoBot::new("test_token");
    /// let result = bot.delete_invoice(528890).await;
    ///
    /// assert!(result.is_ok());
    /// assert!(result.unwrap());
    /// ```
    async fn delete_invoice(&self, invoice_id: u64) -> CryptoBotResult<bool> {
        let params = DeleteInvoiceParams { invoice_id };
        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::DeleteInvoice,
                method: Method::DELETE,
            },
            Some(&params),
        )
        .await
    }

    /// Gets invoices by specified parameters
    ///
    /// # Arguments
    /// * `params` - Parameters for filtering invoices
    ///
    /// # Returns
    /// Returns Result with vector of invoices or CryptoBotError
    ///
    /// # Examples
    /// ```rust
    /// use crypto_pay_api::prelude::*;
    ///
    /// let bot = CryptoBot::new("test_token");
    /// let params = GetInvoicesParams {
    ///     invoice_ids: Some(vec![530195]),
    ///     ..Default::default()
    /// };
    /// let invoices = bot.get_invoices(Some(&params)).await?;
    ///
    /// assert!(!invoices.is_empty());
    /// assert_eq!(invoices.len(), 1);
    /// ```
    async fn get_invoices(
        &self,
        params: Option<&GetInvoicesParams>,
    ) -> CryptoBotResult<Vec<Invoice>> {
        let response: GetInvoicesResponse = self
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetInvoices,
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

    use super::*;
    use crate::models::{CryptoCurrencyCode, CurrencyType};
    use crate::test_utils::test_utils::TestContext;
    use crate::CryptoBot;

    impl TestContext {
        pub fn mock_create_invoice_response(&mut self) -> Mock {
            self.server
                .mock("POST", "/createInvoice")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": {
                            "invoice_id": 528890,
                            "hash": "IVDoTcNBYEfk",
                            "currency_type": "crypto",
                            "asset": "TON",
                            "amount": "10.5",
                            "pay_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                            "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                            "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVDoTcNBYEfk",
                            "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVDoTcNBYEfk",
                            "description": "Test invoice",
                            "status": "active",
                            "created_at": "2025-02-08T12:11:01.341Z",
                            "allow_comments": true,
                            "allow_anonymous": true
                        }
                    })
                    .to_string(),
                )
                .create()
        }

        pub fn mock_get_invoices_response(&mut self) -> Mock {
            self.server
                .mock("GET", "/getInvoices")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(json!({
                    "ok": true,
                    "result": {
                        "items": [
                            {
                                "invoice_id": 528890,
                                "hash": "IVDoTcNBYEfk",
                                "currency_type": "crypto",
                                "asset": "TON",
                                "amount": "10.5",
                                "pay_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                                "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                                "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVDoTcNBYEfk",
                                "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVDoTcNBYEfk",
                                "description": "Test invoice",
                                "status": "active",
                                "created_at": "2025-02-08T12:11:01.341Z",
                                "allow_comments": true,
                                "allow_anonymous": true
                            },
                        ]
                    }
                })
                .to_string(),
            )
            .create()
        }

        pub fn mock_get_invoices_response_with_invoice_ids(&mut self) -> Mock {
            self.server
                .mock("GET", "/getInvoices")
                .match_body(json!({
                    "invoice_ids": "530195"
                }).to_string().as_str()) 
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(json!({
                    "ok": true,
                    "result": {
                        "items": [
                            {
                                "invoice_id": 530195,
                                "hash": "IVcKhSGh244v",
                                "currency_type": "crypto",
                                "asset": "BTC",
                                "amount": "0.5",
                                "pay_url": "https://t.me/CryptoTestnetBot?start=IVcKhSGh244v",
                                "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVcKhSGh244v",
                                "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVcKhSGh244v",
                                "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVcKhSGh244v",
                                "status": "active",
                                "created_at": "2025-02-09T03:46:07.811Z",
                                "allow_comments": true,
                                "allow_anonymous": true
                            }
                        ]
                    }
                })
                .to_string(),
            )
            .create()
        }

        pub fn mock_delete_invoice_response(&mut self) -> Mock {
            self.server
                .mock("DELETE", "/deleteInvoice")
                .with_header("content-type", "application/json")
                .with_header("Crypto-Pay-API-Token", "test_token")
                .with_body(
                    json!({
                        "ok": true,
                        "result": true
                    })
                    .to_string(),
                )
                .create()
        }
    }

    // ! checked
    #[test]
    fn test_create_invoice() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_create_invoice_response();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let params = CreateInvoiceParams {
            currency_type: Some(CurrencyType::Crypto),
            asset: Some(CryptoCurrencyCode::Ton),
            amount: dec!(10.5),
            description: Some("Test invoice".to_string()),
            expires_in: Some(3600),
            ..Default::default()
        };

        let result = ctx.run(async { client.create_invoice(&params).await });

        println!("result: {:?}", result);
        assert!(result.is_ok());

        let invoice = result.unwrap();
        assert_eq!(invoice.amount, dec!(10.5));
        assert_eq!(invoice.asset, Some(CryptoCurrencyCode::Ton));
        assert_eq!(invoice.description, Some("Test invoice".to_string()));
    }

    // ! checked
    #[test]
    fn test_get_invoices_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_invoices_response();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let result = ctx.run(async { client.get_invoices(None).await });

        println!("result:{:?}", result);

        assert!(result.is_ok());

        let invoices = result.unwrap();
        assert!(!invoices.is_empty());
        assert_eq!(invoices.len(), 1);
    }

    // ! checked
    #[test]
    fn test_get_invoices_with_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_invoices_response_with_invoice_ids();
        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);

        let params = GetInvoicesParams {
            invoice_ids: Some(vec![530195]),
            ..Default::default()
        };

        let result = ctx.run(async { client.get_invoices(Some(&params)).await });

        println!("result: {:?}", result);

        assert!(result.is_ok());

        let invoices = result.unwrap();
        assert!(!invoices.is_empty());
        assert_eq!(invoices.len(), 1);
        assert_eq!(invoices[0].invoice_id, 530195);
    }

    // ! checked
    #[test]
    fn test_delete_invoice() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_delete_invoice_response();

        let client = CryptoBot::new_with_base_url("test_token", &ctx.server.url(), None);
        let result = ctx.run(async { client.delete_invoice(528890).await });

        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
