use async_trait::async_trait;
use std::marker::PhantomData;

use rust_decimal::Decimal;

use crate::{
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    models::{
        APIEndpoint, APIMethod, CreateInvoiceParams, CryptoCurrencyCode, CurrencyType, DeleteInvoiceParams,
        FiatCurrencyCode, GetInvoicesParams, GetInvoicesResponse, Invoice, InvoiceStatus, Method, Missing,
        PayButtonName, Set, SwapToAssets,
    },
    validation::{validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext},
};

use super::ExchangeRateAPI;
use super::InvoiceAPI;

pub struct DeleteInvoiceBuilder<'a> {
    client: &'a CryptoBot,
    invoice_id: u64,
}

impl<'a> DeleteInvoiceBuilder<'a> {
    pub fn new(client: &'a CryptoBot, invoice_id: u64) -> Self {
        Self { client, invoice_id }
    }

    /// Executes the request to delete the invoice
    pub async fn execute(self) -> CryptoBotResult<bool> {
        let params = DeleteInvoiceParams {
            invoice_id: self.invoice_id,
        };
        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::DeleteInvoice,
                    method: Method::DELETE,
                },
                Some(&params),
            )
            .await
    }
}

pub struct GetInvoicesBuilder<'a> {
    client: &'a CryptoBot,
    params: GetInvoicesParams,
}

impl<'a> GetInvoicesBuilder<'a> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self {
            client,
            params: GetInvoicesParams::default(),
        }
    }

    /// Set the asset for the invoices.
    /// Optional. Defaults to all currencies.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> Self {
        self.params.asset = Some(asset);
        self
    }

    /// Set the fiat for the invoices.
    /// Optional. Defaults to all currencies.
    pub fn fiat(mut self, fiat: FiatCurrencyCode) -> Self {
        self.params.fiat = Some(fiat);
        self
    }

    /// Set the invoice IDs for the invoices.
    pub fn invoice_ids(mut self, invoice_ids: Vec<u64>) -> Self {
        self.params.invoice_ids = Some(invoice_ids);
        self
    }

    /// Set the status for the invoices.
    /// Optional. Defaults to all statuses.
    pub fn status(mut self, status: InvoiceStatus) -> Self {
        self.params.status = Some(status);
        self
    }

    /// Set the offset for the invoices.
    /// Optional. Offset needed to return a specific subset of invoices.
    /// Defaults to 0.
    pub fn offset(mut self, offset: u32) -> Self {
        self.params.offset = Some(offset);
        self
    }

    /// Set the count for the invoices.
    /// Optional. Number of invoices to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    pub fn count(mut self, count: u16) -> Self {
        self.params.count = Some(count);
        self
    }

    /// Executes the request to get invoices
    pub async fn execute(self) -> CryptoBotResult<Vec<Invoice>> {
        if let Some(count) = self.params.count {
            validate_count(count)?;
        }

        let response: GetInvoicesResponse = self
            .client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::GetInvoices,
                    method: Method::GET,
                },
                Some(&self.params),
            )
            .await?;

        Ok(response.items)
    }
}

pub struct CreateInvoiceBuilder<'a, A = Missing, C = Missing, P = Missing, U = Missing> {
    client: &'a CryptoBot,
    currency_type: Option<CurrencyType>,
    asset: Option<CryptoCurrencyCode>,
    fiat: Option<FiatCurrencyCode>,
    accept_asset: Option<Vec<CryptoCurrencyCode>>,
    amount: Decimal,
    description: Option<String>,
    hidden_message: Option<String>,
    paid_btn_name: Option<PayButtonName>,
    paid_btn_url: Option<String>,
    swap_to: Option<SwapToAssets>,
    payload: Option<String>,
    allow_comments: Option<bool>,
    allow_anonymous: Option<bool>,
    expires_in: Option<u32>,
    _state: PhantomData<(A, C, P, U)>,
}

impl<'a> CreateInvoiceBuilder<'a, Missing, Missing, Missing, Missing> {
    pub fn new(client: &'a CryptoBot) -> Self {
        Self {
            client,
            currency_type: Some(CurrencyType::Crypto),
            asset: None,
            fiat: None,
            accept_asset: None,
            amount: Decimal::ZERO,
            description: None,
            hidden_message: None,
            paid_btn_name: None,
            paid_btn_url: None,
            swap_to: None,
            payload: None,
            allow_comments: None,
            allow_anonymous: None,
            expires_in: None,
            _state: PhantomData,
        }
    }
}

impl<'a, C, P, U> CreateInvoiceBuilder<'a, Missing, C, P, U> {
    /// Set the amount for the invoice.
    pub fn amount(mut self, amount: Decimal) -> CreateInvoiceBuilder<'a, Set, C, P, U> {
        self.amount = amount;
        self.transform()
    }
}

impl<'a, A, P, U> CreateInvoiceBuilder<'a, A, Missing, P, U> {
    /// Set the asset for the invoice, if the currency type is crypto.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> CreateInvoiceBuilder<'a, A, Set, P, U> {
        self.currency_type = Some(CurrencyType::Crypto);
        self.asset = Some(asset);
        self.transform()
    }

    /// Set the fiat for the invoice, if the currency type is fiat.
    pub fn fiat(mut self, fiat: FiatCurrencyCode) -> CreateInvoiceBuilder<'a, A, Set, P, U> {
        self.currency_type = Some(CurrencyType::Fiat);
        self.fiat = Some(fiat);
        self.transform()
    }
}

impl<'a, A, C, U> CreateInvoiceBuilder<'a, A, C, Missing, U> {
    /// Set the paid button name for the invoice.
    pub fn paid_btn_name(mut self, paid_btn_name: PayButtonName) -> CreateInvoiceBuilder<'a, A, C, Set, U> {
        self.paid_btn_name = Some(paid_btn_name);
        self.transform()
    }
}

impl<'a, A, C> CreateInvoiceBuilder<'a, A, C, Set, Missing> {
    /// Set the paid button URL for the invoice.
    pub fn paid_btn_url(mut self, paid_btn_url: impl Into<String>) -> CreateInvoiceBuilder<'a, A, C, Set, Set> {
        self.paid_btn_url = Some(paid_btn_url.into());
        self.transform()
    }
}

impl<'a, A, C, P, U> CreateInvoiceBuilder<'a, A, C, P, U> {
    /// Set the accepted assets for the invoice.
    pub fn accept_asset(mut self, accept_asset: Vec<CryptoCurrencyCode>) -> Self {
        self.accept_asset = Some(accept_asset);
        self
    }

    /// Set the description for the invoice.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the hidden message for the invoice.
    pub fn hidden_message(mut self, hidden_message: impl Into<String>) -> Self {
        self.hidden_message = Some(hidden_message.into());
        self
    }

    /// Set the payload for the invoice.
    pub fn payload(mut self, payload: impl Into<String>) -> Self {
        self.payload = Some(payload.into());
        self
    }

    /// Set the allow comments for the invoice.
    pub fn allow_comments(mut self, allow_comments: bool) -> Self {
        self.allow_comments = Some(allow_comments);
        self
    }

    /// Set the allow anonymous for the invoice.
    pub fn allow_anonymous(mut self, allow_anonymous: bool) -> Self {
        self.allow_anonymous = Some(allow_anonymous);
        self
    }

    /// Set the expiration time for the invoice.
    pub fn expires_in(mut self, expires_in: u32) -> Self {
        self.expires_in = Some(expires_in);
        self
    }

    fn transform<A2, C2, P2, U2>(self) -> CreateInvoiceBuilder<'a, A2, C2, P2, U2> {
        CreateInvoiceBuilder {
            client: self.client,
            currency_type: self.currency_type,
            asset: self.asset,
            fiat: self.fiat,
            accept_asset: self.accept_asset,
            amount: self.amount,
            description: self.description,
            hidden_message: self.hidden_message,
            paid_btn_name: self.paid_btn_name,
            paid_btn_url: self.paid_btn_url,
            swap_to: self.swap_to,
            payload: self.payload,
            allow_comments: self.allow_comments,
            allow_anonymous: self.allow_anonymous,
            expires_in: self.expires_in,
            _state: PhantomData,
        }
    }
}

impl<'a, A, C, P, U> FieldValidate for CreateInvoiceBuilder<'a, A, C, P, U> {
    fn validate(&self) -> CryptoBotResult<()> {
        if self.amount <= Decimal::ZERO {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Amount must be greater than 0".to_string(),
                field: Some("amount".to_string()),
            });
        }

        if let Some(desc) = &self.description {
            if desc.chars().count() > 1024 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "description too long".to_string(),
                    field: Some("description".to_string()),
                });
            }
        }

        if let Some(msg) = &self.hidden_message {
            if msg.chars().count() > 2048 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "hidden_message_too_long".to_string(),
                    field: Some("hidden_message".to_string()),
                });
            }
        }

        if let Some(payload) = &self.payload {
            if payload.chars().count() > 4096 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "payload_too_long".to_string(),
                    field: Some("payload".to_string()),
                });
            }
        }

        if let Some(expires_in) = &self.expires_in {
            if !(&1..=&2678400).contains(&expires_in) {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "expires_in_invalid".to_string(),
                    field: Some("expires_in".to_string()),
                });
            }
        }
        Ok(())
    }
}

#[async_trait]
impl<'a, C: Sync, P: Sync, U: Sync> ContextValidate for CreateInvoiceBuilder<'a, Set, C, P, U> {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        if let Some(asset) = &self.asset {
            validate_amount(&self.amount, asset, ctx).await?;
        }
        Ok(())
    }
}

impl<'a> CreateInvoiceBuilder<'a, Set, Set, Missing, Missing> {
    /// Executes the request to create the invoice
    pub async fn execute(self) -> CryptoBotResult<Invoice> {
        self.validate()?;

        let exchange_rates = self.client.get_exchange_rates().execute().await?;
        let ctx = ValidationContext { exchange_rates };
        self.validate_with_context(&ctx).await?;

        let params = CreateInvoiceParams {
            currency_type: self.currency_type,
            asset: self.asset,
            fiat: self.fiat,
            accept_asset: self.accept_asset,
            amount: self.amount,
            description: self.description,
            hidden_message: self.hidden_message,
            paid_btn_name: self.paid_btn_name,
            paid_btn_url: self.paid_btn_url,
            swap_to: self.swap_to,
            payload: self.payload,
            allow_comments: self.allow_comments,
            allow_anonymous: self.allow_anonymous,
            expires_in: self.expires_in,
        };

        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::CreateInvoice,
                    method: Method::POST,
                },
                Some(&params),
            )
            .await
    }
}

impl<'a> CreateInvoiceBuilder<'a, Set, Set, Set, Set> {
    /// Executes the request to create the invoice
    pub async fn execute(self) -> CryptoBotResult<Invoice> {
        self.validate()?;

        if let Some(url) = &self.paid_btn_url {
            if !url.starts_with("https://") && !url.starts_with("http://") {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Format,
                    message: "paid_btn_url_invalid".to_string(),
                    field: Some("paid_btn_url".to_string()),
                });
            }
        }

        let exchange_rates = self.client.get_exchange_rates().execute().await?;
        let ctx = ValidationContext { exchange_rates };
        self.validate_with_context(&ctx).await?;

        let params = CreateInvoiceParams {
            currency_type: self.currency_type,
            asset: self.asset,
            fiat: self.fiat,
            accept_asset: self.accept_asset,
            amount: self.amount,
            description: self.description,
            hidden_message: self.hidden_message,
            paid_btn_name: self.paid_btn_name,
            paid_btn_url: self.paid_btn_url,
            swap_to: self.swap_to,
            payload: self.payload,
            allow_comments: self.allow_comments,
            allow_anonymous: self.allow_anonymous,
            expires_in: self.expires_in,
        };

        self.client
            .make_request(
                &APIMethod {
                    endpoint: APIEndpoint::CreateInvoice,
                    method: Method::POST,
                },
                Some(&params),
            )
            .await
    }
}

#[async_trait]
impl InvoiceAPI for CryptoBot {
    /// Creates a new cryptocurrency invoice
    ///
    /// An invoice is a request for cryptocurrency payment with a specific amount
    /// and currency. Once created, the invoice can be paid by any user.
    ///
    /// # Returns
    /// * `CreateInvoiceBuilder` - A builder to construct the invoice parameters
    fn create_invoice(&self) -> CreateInvoiceBuilder<'_> {
        CreateInvoiceBuilder::new(self)
    }

    fn delete_invoice(&self, invoice_id: u64) -> DeleteInvoiceBuilder<'_> {
        DeleteInvoiceBuilder::new(self, invoice_id)
    }

    /// Gets a list of invoices with optional filtering
    ///
    /// Retrieves all invoices matching the specified filter parameters.
    /// If no parameters are provided, returns all invoices.
    ///
    /// # Returns
    /// * `GetInvoicesBuilder` - A builder to construct the filter parameters
    fn get_invoices(&self) -> GetInvoicesBuilder<'_> {
        GetInvoicesBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Mock;
    use rust_decimal_macros::dec;
    use serde_json::json;

    use super::*;
    use crate::models::{CryptoCurrencyCode, PayButtonName, SwapToAssets};
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
            client
                .create_invoice()
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(10.5))
                .description("Test invoice".to_string())
                .expires_in(3600)
                .execute()
                .await
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
        let result = ctx.run(async { client.get_invoices().execute().await });

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

        let result = ctx.run(async { client.get_invoices().invoice_ids(vec![530195]).execute().await });

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

        let result = ctx.run(async { client.delete_invoice(528890).execute().await });

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_get_invoices_with_all_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_get_invoices_response();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .get_invoices()
                .asset(CryptoCurrencyCode::Ton)
                .fiat(FiatCurrencyCode::Usd)
                .status(InvoiceStatus::Paid)
                .offset(10)
                .count(50)
                .execute()
                .await
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_invoices_invalid_count() {
        let ctx = TestContext::new();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async { client.get_invoices().count(0).execute().await });

        assert!(result.is_err());
        match result {
            Err(CryptoBotError::ValidationError { kind, .. }) => {
                assert_eq!(kind, ValidationErrorKind::Range);
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_create_invoice_with_all_optional_params() {
        let mut ctx = TestContext::new();
        let _m = ctx.mock_exchange_rates_response();
        let _m = ctx.mock_create_invoice_response();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .create_invoice()
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(10.5))
                .description("Test".to_string())
                .hidden_message("Hidden".to_string())
                .paid_btn_name(PayButtonName::ViewItem)
                .paid_btn_url("https://example.com".to_string())
                .payload("payload".to_string())
                .allow_comments(true)
                .allow_anonymous(false)
                .expires_in(3600)
                .execute()
                .await
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_swap_to_assets_serialization() {
        let serialized = serde_json::to_string(&SwapToAssets::Ton).unwrap();
        assert_eq!(serialized, "\"TON\"");

        let deserialized: SwapToAssets = serde_json::from_str("\"USDT\"").unwrap();
        assert_eq!(deserialized, SwapToAssets::Usdt);
    }

    #[test]
    fn test_invoice_swap_fields_serialization() {
        let invoice: Invoice = serde_json::from_value(json!({
            "invoice_id": 123,
            "hash": "hash-value",
            "currency_type": "crypto",
            "asset": "TON",
            "amount": "10.00",
            "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=hash-value",
            "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-hash-value",
            "web_app_invoice_url": "https://testnet-app.send.tg/invoices/hash-value",
            "status": "paid",
            "allow_comments": true,
            "allow_anonymous": false,
            "created_at": "2025-02-08T12:11:01.341Z",
            "swap_to": "USDT",
            "is_swapped": "true",
            "swapped_uid": "swap-uid",
            "swapped_to": "USDT",
            "swapped_rate": "1.50",
            "swapped_output": "100.00",
            "swapped_usd_amount": "1500.00",
            "swapped_usd_rate": "1.50"
        }))
        .unwrap();

        assert_eq!(invoice.swapped_usd_amount, Some(dec!(1500.00))); // 1500.00
        assert_eq!(invoice.swapped_usd_rate, Some(dec!(1.50))); // 1.50
        assert_eq!(invoice.swap_to, Some(SwapToAssets::Usdt));
        assert_eq!(invoice.swapped_to, Some(SwapToAssets::Usdt));
    }

    #[test]
    fn test_create_invoice_rejects_negative_amount() {
        let ctx = TestContext::new();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let builder = client.create_invoice().asset(CryptoCurrencyCode::Ton).amount(dec!(-1));

        let result = builder.validate();
        assert!(result.is_err());
        match result {
            Err(CryptoBotError::ValidationError { field, .. }) => assert_eq!(field, Some("amount".to_string())),
            _ => panic!("Expected validation error for negative amount"),
        }
    }

    #[test]
    fn test_create_invoice_rejects_description_too_long() {
        let ctx = TestContext::new();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let long_description = "a".repeat(1_025);
        let builder = client
            .create_invoice()
            .asset(CryptoCurrencyCode::Ton)
            .amount(dec!(1))
            .description(long_description);

        let result = builder.validate();
        assert!(result.is_err());
        match result {
            Err(CryptoBotError::ValidationError { field, .. }) => {
                assert_eq!(field, Some("description".to_string()))
            }
            _ => panic!("Expected validation error for long description"),
        }
    }

    #[test]
    fn test_create_invoice_invalid_paid_button_url() {
        let ctx = TestContext::new();
        let client = CryptoBot::builder()
            .api_token("test_token")
            .base_url(ctx.server.url())
            .build()
            .unwrap();

        let result = ctx.run(async {
            client
                .create_invoice()
                .asset(CryptoCurrencyCode::Ton)
                .amount(dec!(5))
                .paid_btn_name(PayButtonName::ViewItem)
                .paid_btn_url("ftp://example.com")
                .execute()
                .await
        });

        assert!(result.is_err());
        match result {
            Err(CryptoBotError::ValidationError { field, .. }) => assert_eq!(field, Some("paid_btn_url".to_string())),
            _ => panic!("Expected validation error for invalid paid_btn_url"),
        }
    }
}
