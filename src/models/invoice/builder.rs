use std::marker::PhantomData;

use rust_decimal::Decimal;

use crate::{
    api::ExchangeRateAPI,
    client::CryptoBot,
    error::{CryptoBotError, CryptoBotResult, ValidationErrorKind},
    models::{CryptoCurrencyCode, CurrencyType, FiatCurrencyCode, Missing, PayButtonName, Set},
    validation::{
        validate_amount, validate_count, ContextValidate, FieldValidate, ValidationContext,
    },
};

use super::{CreateInvoiceParams, GetInvoicesParams, InvoiceStatus};

/* #region GetInvoicesParamsBuilder */

#[derive(Debug, Default)]
pub struct GetInvoicesParamsBuilder {
    pub asset: Option<CryptoCurrencyCode>,
    pub fiat: Option<FiatCurrencyCode>,
    pub invoice_ids: Option<Vec<u64>>,
    pub status: Option<InvoiceStatus>,
    pub offset: Option<u32>,
    pub count: Option<u16>,
}

impl GetInvoicesParamsBuilder {
    /// Create a new `GetInvoicesParamsBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the asset for the invoices.
    /// Optional. Defaults to all currencies.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> Self {
        self.asset = Some(asset);
        self
    }

    /// Set the fiat for the invoices.
    /// Optional. Defaults to all currencies.
    pub fn fiat(mut self, fiat: FiatCurrencyCode) -> Self {
        self.fiat = Some(fiat);
        self
    }

    /// Set the invoice IDs for the invoices.
    pub fn invoice_ids(mut self, invoice_ids: Vec<u64>) -> Self {
        self.invoice_ids = Some(invoice_ids);
        self
    }

    /// Set the status for the invoices.
    /// Optional. Defaults to all statuses.
    pub fn status(mut self, status: InvoiceStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the offset for the invoices.
    /// Optional. Offset needed to return a specific subset of invoices.
    /// Defaults to 0.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the count for the invoices.
    /// Optional. Number of invoices to be returned. Values between 1-1000 are accepted.
    /// Defaults to 100.
    pub fn count(mut self, count: u16) -> Self {
        self.count = Some(count);
        self
    }
}

impl FieldValidate for GetInvoicesParamsBuilder {
    fn validate(&self) -> CryptoBotResult<()> {
        if let Some(count) = &self.count {
            validate_count(*count)?;
        }
        Ok(())
    }
}

impl GetInvoicesParamsBuilder {
    pub fn build(self) -> CryptoBotResult<GetInvoicesParams> {
        self.validate()?;

        Ok(GetInvoicesParams {
            asset: self.asset,
            fiat: self.fiat,
            invoice_ids: self.invoice_ids,
            status: self.status,
            offset: self.offset,
            count: self.count,
        })
    }
}
/* #endregion */

/* #region CreateInvoiceParamsBuilder */

// A - Asset, C - CurrencyType (Crypto or Fiat), P - PayButtonName, U - PayButtonUrl
#[derive(Debug)]
pub struct CreateInvoiceParamsBuilder<A = Missing, C = Missing, P = Missing, U = Missing> {
    pub currency_type: Option<CurrencyType>,
    pub asset: Option<CryptoCurrencyCode>,
    pub fiat: Option<FiatCurrencyCode>,
    pub accept_asset: Option<Vec<CryptoCurrencyCode>>,
    pub amount: Decimal,
    pub description: Option<String>,
    pub hidden_message: Option<String>,
    pub paid_btn_name: Option<PayButtonName>,
    pub paid_btn_url: Option<String>,
    pub payload: Option<String>,
    pub allow_comments: Option<bool>,
    pub allow_anonymous: Option<bool>,
    pub expires_in: Option<u32>,
    _state: PhantomData<(A, C, P, U)>,
}

impl CreateInvoiceParamsBuilder<Missing, Missing, Missing, Missing> {
    /// Create a new `CreateInvoiceParamsBuilder` with default values.
    pub fn new() -> Self {
        Self {
            currency_type: Some(CurrencyType::Crypto),
            asset: None,
            fiat: None,
            accept_asset: None,
            amount: Decimal::ZERO,
            description: None,
            hidden_message: None,
            paid_btn_name: None,
            paid_btn_url: None,
            payload: None,
            allow_comments: None,
            allow_anonymous: None,
            expires_in: None,
            _state: PhantomData,
        }
    }
}

impl<C, P, U> CreateInvoiceParamsBuilder<Missing, C, P, U> {
    /// Set the amount for the invoice.
    pub fn amount(mut self, amount: Decimal) -> CreateInvoiceParamsBuilder<Set, C, P, U> {
        self.amount = amount;
        self.transform()
    }
}

impl<A, P, U> CreateInvoiceParamsBuilder<A, Missing, P, U> {
    /// Set the asset for the invoice, if the currency type is crypto.
    pub fn asset(mut self, asset: CryptoCurrencyCode) -> CreateInvoiceParamsBuilder<A, Set, P, U> {
        self.currency_type = Some(CurrencyType::Crypto);
        self.asset = Some(asset);
        self.transform()
    }

    /// Set the fiat for the invoice, if the currency type is fiat.
    pub fn fiat(mut self, fiat: FiatCurrencyCode) -> CreateInvoiceParamsBuilder<A, Set, P, U> {
        self.currency_type = Some(CurrencyType::Fiat);
        self.fiat = Some(fiat);
        self.transform()
    }
}

impl<A, C, U> CreateInvoiceParamsBuilder<A, C, Missing, U> {
    /// Set the paid button name for the invoice.
    /// Optional. Label of the button which will be presented to a user after the invoice is paid.
    /// Supported names:
    /// viewItem – "View Item",
    /// openChannel – "View Channel",
    /// openBot – "Open Bot",
    /// callback – "Return to the bot"  
    pub fn paid_btn_name(
        mut self,
        paid_btn_name: PayButtonName,
    ) -> CreateInvoiceParamsBuilder<A, C, Set, U> {
        self.paid_btn_name = Some(paid_btn_name);
        self.transform()
    }
}

impl<A, C> CreateInvoiceParamsBuilder<A, C, Set, Missing> {
    /// Set the paid button URL for the invoice.
    /// Optional. Required if paid_btn_name is specified. URL opened using the button which will be presented to a user after the invoice is paid.
    /// You can set any callback link (for example, a success link or link to homepage).
    /// Starts with https or http.
    pub fn paid_btn_url(
        mut self,
        paid_btn_url: impl Into<String>,
    ) -> CreateInvoiceParamsBuilder<A, C, Set, Set> {
        self.paid_btn_url = Some(paid_btn_url.into());
        self.transform()
    }
}

impl<A, C, P, U> CreateInvoiceParamsBuilder<A, C, P, U> {
    /// Set the accepted assets for the invoice.
    /// Optional. Defaults to all currencies.
    pub fn accept_asset(mut self, accept_asset: Vec<CryptoCurrencyCode>) -> Self {
        self.accept_asset = Some(accept_asset);
        self
    }

    /// Set the description for the invoice.
    /// Optional. Description for the invoice. User will see this description when they pay the invoice.
    /// Up to 1024 characters.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the hidden message for the invoice.
    /// Optional. Text of the message which will be presented to a user after the invoice is paid.
    /// Up to 2048 characters.
    pub fn hidden_message(mut self, hidden_message: impl Into<String>) -> Self {
        self.hidden_message = Some(hidden_message.into());
        self
    }

    /// Set the payload for the invoice.
    /// Optional. Any data you want to attach to the invoice (for example, user ID, payment ID, ect).
    /// Up to 4kb.
    pub fn payload(mut self, payload: impl Into<String>) -> Self {
        self.payload = Some(payload.into());
        self
    }

    /// Set the allow comments for the invoice.
    /// Optional. Allow a user to add a comment to the payment.
    /// Defaults to true.
    pub fn allow_comments(mut self, allow_comments: bool) -> Self {
        self.allow_comments = Some(allow_comments);
        self
    }

    /// Set the allow anonymous for the invoice.
    /// Optional. Allow a user to pay the invoice anonymously.
    /// Defaults to true.
    pub fn allow_anonymous(mut self, allow_anonymous: bool) -> Self {
        self.allow_anonymous = Some(allow_anonymous);
        self
    }

    /// Set the expiration time for the invoice.
    /// Optional. You can set a payment time limit for the invoice in seconds.
    /// Values between 1-2678400 are accepted.
    pub fn expires_in(mut self, expires_in: u32) -> Self {
        self.expires_in = Some(expires_in);
        self
    }

    fn transform<A2, C2, P2, U2>(self) -> CreateInvoiceParamsBuilder<A2, C2, P2, U2> {
        CreateInvoiceParamsBuilder {
            currency_type: self.currency_type,
            asset: self.asset,
            fiat: self.fiat,
            accept_asset: self.accept_asset,
            amount: self.amount,
            description: self.description,
            hidden_message: self.hidden_message,
            paid_btn_name: self.paid_btn_name,
            paid_btn_url: self.paid_btn_url,
            payload: self.payload,
            allow_comments: self.allow_comments,
            allow_anonymous: self.allow_anonymous,
            expires_in: self.expires_in,
            _state: PhantomData,
        }
    }
}

impl<A, C, P, U> FieldValidate for CreateInvoiceParamsBuilder<A, C, P, U> {
    fn validate(&self) -> CryptoBotResult<()> {
        // Amount > 0
        if self.amount < Decimal::ZERO {
            return Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                message: "Amount must be greater than 0".to_string(),
                field: Some("amount".to_string()),
            });
        }

        // Description <= 1024 chars
        if let Some(desc) = &self.description {
            if desc.chars().count() > 1024 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "description_too_long".to_string(),
                    field: Some("description".to_string()),
                });
            }
        }

        // Hidden message <= 2048 chars
        if let Some(msg) = &self.hidden_message {
            if msg.chars().count() > 2048 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "hidden_message_too_long".to_string(),
                    field: Some("hidden_message".to_string()),
                });
            }
        }

        // Payload up to 4kb
        if let Some(payload) = &self.payload {
            if payload.chars().count() > 4096 {
                return Err(CryptoBotError::ValidationError {
                    kind: ValidationErrorKind::Range,
                    message: "payload_too_long".to_string(),
                    field: Some("payload".to_string()),
                });
            }
        }

        // ExpiresIn between 1 and 2678400 seconds
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

impl CreateInvoiceParamsBuilder<Set, Set, Missing, Missing> {
    pub async fn build(self, client: &CryptoBot) -> CryptoBotResult<CreateInvoiceParams> {
        self.validate()?;

        let exchange_rates = client.get_exchange_rates().await?;
        let ctx = ValidationContext { exchange_rates };
        self.validate_with_context(&ctx).await?;

        Ok(CreateInvoiceParams {
            currency_type: self.currency_type,
            asset: self.asset,
            fiat: self.fiat,
            accept_asset: self.accept_asset,
            amount: self.amount,
            description: self.description,
            hidden_message: self.hidden_message,
            paid_btn_name: self.paid_btn_name,
            paid_btn_url: self.paid_btn_url,
            payload: self.payload,
            allow_comments: self.allow_comments,
            allow_anonymous: self.allow_anonymous,
            expires_in: self.expires_in,
        })
    }
}

impl CreateInvoiceParamsBuilder<Set, Set, Set, Set> {
    pub async fn build(self, client: &CryptoBot) -> CryptoBotResult<CreateInvoiceParams> {
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

        let rates = client.get_exchange_rates().await?;
        let ctx = ValidationContext {
            exchange_rates: rates,
        };
        self.validate_with_context(&ctx).await?;

        Ok(CreateInvoiceParams {
            currency_type: self.currency_type,
            asset: self.asset,
            fiat: self.fiat,
            accept_asset: self.accept_asset,
            amount: self.amount,
            description: self.description,
            hidden_message: self.hidden_message,
            paid_btn_name: self.paid_btn_name,
            paid_btn_url: self.paid_btn_url,
            payload: self.payload,
            allow_comments: self.allow_comments,
            allow_anonymous: self.allow_anonymous,
            expires_in: self.expires_in,
        })
    }
}

#[async_trait::async_trait]
impl<C: Sync, P: Sync, U: Sync> ContextValidate for CreateInvoiceParamsBuilder<Set, C, P, U> {
    async fn validate_with_context(&self, ctx: &ValidationContext) -> CryptoBotResult<()> {
        if let Some(asset) = &self.asset {
            println!("Validating amount");
            validate_amount(&self.amount, asset, ctx).await?;
        }
        Ok(())
    }
}

/* #endregion */

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_get_invoices_params_builder() {
        let params = GetInvoicesParamsBuilder::new().count(100).build().unwrap();
        assert_eq!(params.count, Some(100));
        assert_eq!(params.offset, None);
        assert_eq!(params.invoice_ids, None);
        assert_eq!(params.status, None);
        assert_eq!(params.asset, None);
        assert_eq!(params.fiat, None);
    }

    #[test]
    fn test_get_invoices_params_builder_invalid_count() {
        let result = GetInvoicesParamsBuilder::new().count(1001).build();

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "count"
        ));
    }

    #[tokio::test]
    async fn test_create_invoice_params_builder() {
        let client = CryptoBot::test_client();
        let params = CreateInvoiceParamsBuilder::new()
            .amount(Decimal::from(100))
            .asset(CryptoCurrencyCode::Ton)
            .build(&client)
            .await
            .unwrap();

        assert_eq!(params.amount, Decimal::from(100));
        assert_eq!(params.currency_type, Some(CurrencyType::Crypto));
        assert_eq!(params.asset, Some(CryptoCurrencyCode::Ton));
        assert_eq!(params.fiat, None);
        assert_eq!(params.accept_asset, None);
        assert_eq!(params.description, None);
        assert_eq!(params.hidden_message, None);
    }

    #[tokio::test]
    async fn test_create_invoice_params_builder_invalid_amount() {
        let client = CryptoBot::test_client();
        let result = CreateInvoiceParamsBuilder::new()
            .amount(Decimal::from(-100))
            .asset(CryptoCurrencyCode::Ton)
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "amount"
        ));

        let result = CreateInvoiceParamsBuilder::new()
            .amount(dec!(10000))
            .asset(CryptoCurrencyCode::Ton)
            .build(&client)
            .await;

        println!("{:?}", result);

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "amount"
        ));
    }

    #[tokio::test]
    async fn test_create_invoice_params_builder_invalid_description() {
        let client = CryptoBot::test_client();
        let result = CreateInvoiceParamsBuilder::new()
            .amount(Decimal::from(100))
            .fiat(FiatCurrencyCode::Usd)
            .description("a".repeat(1025))
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "description"
        ));
    }

    #[tokio::test]
    async fn test_create_invoice_params_builder_invalid_hidden_message() {
        let client = CryptoBot::test_client();
        let result = CreateInvoiceParamsBuilder::new()
            .amount(dec!(10.0))
            .asset(CryptoCurrencyCode::Ton)
            .hidden_message("a".repeat(2049))
            .build(&client)
            .await;

        assert!(matches!(
                result,
                Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "hidden_message"
        ));
    }

    #[tokio::test]
    async fn test_create_invoice_params_builder_invalid_payload() {
        let client = CryptoBot::test_client();
        let result = CreateInvoiceParamsBuilder::new()
            .amount(dec!(10.0))
            .fiat(FiatCurrencyCode::Usd)
            .payload("a".repeat(4097))
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "payload"
        ));
    }

    #[tokio::test]
    async fn test_create_invoice_params_builder_invalid_expires_in() {
        let client = CryptoBot::test_client();
        let result = CreateInvoiceParamsBuilder::new()
            .amount(dec!(10.0))
            .fiat(FiatCurrencyCode::Usd)
            .expires_in(0)
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Range,
                field: Some(field),
                ..
            }) if field == "expires_in"
        ));
    }

    #[tokio::test]
    async fn test_create_invoice_params_builder_invalid_paid_btn_url() {
        let client = CryptoBot::test_client();
        let result = CreateInvoiceParamsBuilder::new()
            .amount(dec!(10.0))
            .fiat(FiatCurrencyCode::Usd)
            .paid_btn_name(PayButtonName::OpenBot)
            .paid_btn_url("invalid_url")
            .build(&client)
            .await;

        assert!(matches!(
            result,
            Err(CryptoBotError::ValidationError {
                kind: ValidationErrorKind::Format,
                field: Some(field),
                ..
            }) if field == "paid_btn_url"
        ));
    }
}
