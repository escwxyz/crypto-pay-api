use async_trait::async_trait;

use crate::{
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult},
    models::{
        APIEndpoint, APIMethod, CreateInvoiceParams, DeleteInvoiceParams, GetInvoicesParams,
        GetInvoicesResponse, Invoice, Method,
    },
};

use super::InvoiceAPI;

#[async_trait]
impl InvoiceAPI for CryptoBot {
    /// Creates a new cryptocurrency invoice
    ///
    /// An invoice is a request for cryptocurrency payment with a specific amount
    /// and currency. Once created, the invoice can be paid by any user.
    ///
    /// # Arguments
    /// * `params` - Parameters for creating the invoice. See [`CreateInvoiceParams`] for details.
    ///
    /// # Returns
    /// * `Ok(Invoice)` - The created invoice
    /// * `Err(CryptoBotError)` - If validation fails or the request fails
    ///
    /// # Errors
    /// This method will return an error if:
    /// * The parameters are invalid (e.g., negative amount)
    /// * The currency is not supported
    /// * The API request fails
    /// * The exchange rate validation fails (for paid_amount/paid_currency)
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///     
    ///     let params = CreateInvoiceParamsBuilder::new()
    ///         .asset(CryptoCurrencyCode::Ton)
    ///         .amount(dec!(10.5))
    ///         .description("Payment for service")
    ///         .paid_btn_name(PayButtonName::ViewItem)
    ///         .paid_btn_url("https://example.com/order/123")
    ///         .build(&client)
    ///         .await
    ///         .unwrap();
    ///     
    ///     let invoice = client.create_invoice(&params).await?;
    ///     println!("Invoice created: {}", invoice.amount);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [Invoice](struct.Invoice.html) - The structure representing an invoice
    /// * [CreateInvoiceParams](struct.CreateInvoiceParams.html) - The parameters for creating an invoice
    async fn create_invoice(
        &self,
        params: &CreateInvoiceParams,
    ) -> Result<Invoice, CryptoBotError> {
        self.make_request(
            &APIMethod {
                endpoint: APIEndpoint::CreateInvoice,
                method: Method::POST,
            },
            Some(params),
        )
        .await
    }

    /// Deletes an existing invoice
    ///
    /// Once deleted, the invoice becomes invalid and cannot be paid.
    /// This is useful for cancelling unpaid invoices.
    ///
    /// # Arguments
    /// * `invoice_id` - The unique identifier of the invoice to delete
    ///
    /// # Returns
    /// * `Ok(true)` - If the invoice was successfully deleted
    /// * `Err(CryptoBotError)` - If the invoice doesn't exist or the request fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///     
    ///     match client.delete_invoice(12345).await {
    ///         Ok(_) => println!("Invoice deleted successfully"),
    ///         Err(e) => eprintln!("Failed to delete invoice: {}", e),
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [Invoice](struct.Invoice.html) - The structure representing an invoice
    /// * [DeleteInvoiceParams](struct.DeleteInvoiceParams.html) - The parameters for deleting an invoice
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

    /// Gets a list of invoices with optional filtering
    ///
    /// Retrieves all invoices matching the specified filter parameters.
    /// If no parameters are provided, returns all invoices.
    ///
    /// # Arguments
    /// * `params` - Optional filter parameters. See [`GetInvoicesParams`] for available filters.
    ///
    /// # Returns
    /// * `Ok(Vec<Invoice>)` - List of invoices matching the filter criteria
    /// * `Err(CryptoBotError)` - If the parameters are invalid or the request fails
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///
    ///     let params = GetInvoicesParamsBuilder::new()
    ///         .asset(CryptoCurrencyCode::Ton)
    ///         .status(InvoiceStatus::Paid)
    ///         .build()
    ///         .unwrap();
    ///
    ///     let invoices = client.get_invoices(Some(&params)).await?;
    ///
    ///     for invoice in invoices {
    ///         println!("Invoice #{}: {} {} (paid at: {})",
    ///             invoice.invoice_id,
    ///             invoice.amount,
    ///             invoice.asset.unwrap(),
    ///             invoice.paid_at.unwrap_or_default()
    ///         );
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    /// * [Invoice](struct.Invoice.html) - The structure representing an invoice
    /// * [GetInvoicesParams](struct.GetInvoicesParams.html) - Available filter parameters
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
    use crate::models::{CreateInvoiceParamsBuilder, CryptoCurrencyCode, GetInvoicesParamsBuilder};
    use crate::utils::test_utils::TestContext;

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
                .match_body(json!({ "invoice_ids": "530195"}).to_string().as_str())
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

    #[test]
    fn test_create_invoice() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_create_invoice_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            let params = CreateInvoiceParamsBuilder::new()
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(10.5))
                .description("Test invoice".to_string())
                .expires_in(3600)
                .build(&client)
                .await
                .unwrap();

            client.create_invoice(&params).await
        });

        println!("result: {:?}", result);
        assert!(result.is_ok());

        let invoice = result.unwrap();
        assert_eq!(invoice.amount, dec!(10.5));
        assert_eq!(invoice.asset, Some(CryptoCurrencyCode::Ton));
        assert_eq!(invoice.description, Some("Test invoice".to_string()));
    }

    #[test]
    fn test_get_invoices_without_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_invoices_response();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();
        let result = ctx.run(async { client.get_invoices(None).await });

        println!("result:{:?}", result);

        assert!(result.is_ok());

        let invoices = result.unwrap();
        assert!(!invoices.is_empty());
        assert_eq!(invoices.len(), 1);
    }

    #[test]
    fn test_get_invoices_with_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_invoices_response_with_invoice_ids();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();
        let params = GetInvoicesParamsBuilder::new()
            .invoice_ids(vec![530195])
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_invoices(Some(&params)).await });

        println!("result: {:?}", result);

        assert!(result.is_ok());

        let invoices = result.unwrap();
        assert!(!invoices.is_empty());
        assert_eq!(invoices.len(), 1);
        assert_eq!(invoices[0].invoice_id, 530195);
    }

    #[test]
    fn test_delete_invoice() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_delete_invoice_response();

        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.delete_invoice(528890).await });

        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
