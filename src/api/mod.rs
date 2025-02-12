mod balance;
mod check;
mod exchange;
mod invoice;
mod misc;
mod transfer;

use async_trait::async_trait;

use crate::{
    error::CryptoBotResult,
    models::{
        AppStats, Balance, Check, CreateCheckParams, CreateInvoiceParams, Currency, ExchangeRate, GetChecksParams,
        GetInvoicesParams, GetMeResponse, GetStatsParams, GetTransfersParams, Invoice, Transfer, TransferParams,
    },
};

#[async_trait]
pub trait MiscAPI {
    async fn get_me(&self) -> CryptoBotResult<GetMeResponse>;
    async fn get_currencies(&self) -> CryptoBotResult<Vec<Currency>>;
    async fn get_stats(&self, params: Option<&GetStatsParams>) -> CryptoBotResult<AppStats>;
}

#[async_trait]
pub trait BalanceAPI {
    async fn get_balance(&self) -> CryptoBotResult<Vec<Balance>>;
}

#[async_trait]
pub trait CheckAPI {
    async fn create_check(&self, params: &CreateCheckParams) -> CryptoBotResult<Check>;
    async fn delete_check(&self, check_id: u64) -> CryptoBotResult<bool>;
    async fn get_checks(&self, params: Option<&GetChecksParams>) -> CryptoBotResult<Vec<Check>>;
}

#[async_trait]
pub trait ExchangeRateAPI {
    async fn get_exchange_rates(&self) -> CryptoBotResult<Vec<ExchangeRate>>;
}
#[async_trait]
pub trait TransferAPI {
    async fn transfer(&self, params: &TransferParams) -> CryptoBotResult<Transfer>;
    async fn get_transfers(&self, params: Option<&GetTransfersParams>) -> CryptoBotResult<Vec<Transfer>>;
}

#[async_trait]
pub trait InvoiceAPI {
    async fn create_invoice(&self, params: &CreateInvoiceParams) -> CryptoBotResult<Invoice>;
    async fn delete_invoice(&self, invoice_id: u64) -> CryptoBotResult<bool>;
    async fn get_invoices(&self, params: Option<&GetInvoicesParams>) -> CryptoBotResult<Vec<Invoice>>;
}
