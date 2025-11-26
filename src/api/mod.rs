mod balance;
mod check;
mod exchange;
mod invoice;
mod misc;
mod transfer;

use async_trait::async_trait;

#[async_trait]
pub trait MiscAPI {
    fn get_me(&self) -> misc::GetMeBuilder<'_>;
    fn get_currencies(&self) -> misc::GetCurrenciesBuilder<'_>;
    fn get_stats(&self) -> misc::GetStatsBuilder<'_>;
}

#[async_trait]
pub trait BalanceAPI {
    fn get_balance(&self) -> balance::GetBalanceBuilder<'_>;
}

#[async_trait]
pub trait CheckAPI {
    fn create_check(&self) -> check::CreateCheckBuilder<'_>;
    fn delete_check(&self, check_id: u64) -> check::DeleteCheckBuilder<'_>;
    fn get_checks(&self) -> check::GetChecksBuilder<'_>;
}

#[async_trait]
pub trait ExchangeRateAPI {
    fn get_exchange_rates(&self) -> exchange::GetExchangeRatesBuilder<'_>;
}
#[async_trait]
pub trait TransferAPI {
    fn transfer(&self) -> transfer::TransferBuilder<'_>;
    fn get_transfers(&self) -> transfer::GetTransfersBuilder<'_>;
}

#[async_trait]
pub trait InvoiceAPI {
    fn create_invoice(&self) -> invoice::CreateInvoiceBuilder<'_>;
    fn delete_invoice(&self, invoice_id: u64) -> invoice::DeleteInvoiceBuilder<'_>;
    fn get_invoices(&self) -> invoice::GetInvoicesBuilder<'_>;
}
