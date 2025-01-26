use chrono::DateTime;

use crate::{
    balance::Balance,
    exchange::ExchangeRate,
    invoice::{CreateInvoiceParams, GetInvoicesParams, Invoice, InvoiceStatus},
    model::CryptoCurrencyCode,
    transfer::{Transfer, TransferParams},
    CryptoBotError,
};

#[async_trait::async_trait]
pub trait BalanceClient {
    async fn get_balance(&self) -> Result<Vec<Balance>, CryptoBotError>;
}

#[async_trait::async_trait]
pub trait CheckClient {}

#[async_trait::async_trait]
pub trait ExchangeClient {
    async fn get_exchange_rates(&self) -> Result<Vec<ExchangeRate>, CryptoBotError>;
}
#[async_trait::async_trait]
pub trait TransferClient {
    async fn transfer(&self, params: &TransferParams) -> Result<Transfer, CryptoBotError>;
    async fn get_transfers(
        &self,
        asset: Option<CryptoCurrencyCode>,
        transfer_ids: Option<Vec<i64>>,
        offset: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<Transfer>, CryptoBotError>;
}

#[async_trait::async_trait]
pub trait InvoiceClient {
    /// Creates a new invoice
    async fn create_invoice(&self, params: &CreateInvoiceParams)
        -> Result<Invoice, CryptoBotError>;

    /// Deletes an invoice
    async fn delete_invoice(&self, invoice_id: i64) -> Result<bool, CryptoBotError>;

    /// Gets invoices by specified parameters
    async fn get_invoices(
        &self,
        params: &GetInvoicesParams,
    ) -> Result<Vec<Invoice>, CryptoBotError>;

    /// Gets information about a specific invoice
    async fn get_invoice(&self, invoice_id: i64) -> Result<Invoice, CryptoBotError>;

    /// Confirms paid status of an invoice
    async fn confirm_paid_invoice(
        &self,
        invoice_id: i64,
        paid_at: DateTime<chrono::Utc>,
    ) -> Result<Invoice, CryptoBotError>;

    /// Checks if an invoice is paid
    async fn is_invoice_paid(&self, invoice_id: i64) -> Result<bool, CryptoBotError>;

    /// Checks if an invoice is expired
    async fn is_invoice_expired(&self, invoice_id: i64) -> Result<bool, CryptoBotError>;

    /// Gets the status of an invoice
    async fn get_invoice_status(&self, invoice_id: i64) -> Result<InvoiceStatus, CryptoBotError>;
}
