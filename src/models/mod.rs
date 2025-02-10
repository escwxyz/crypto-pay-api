mod app_stats;
mod balance;
mod check;
mod currency;
mod exchange_rate;
mod invoice;
mod response;
mod transfer;
mod webhook;

pub use app_stats::*;
pub use balance::*;
pub use check::*;
pub use currency::*;
pub use exchange_rate::*;
pub use invoice::*;
pub use response::*;
use serde::{Deserialize, Serialize};
pub use transfer::*;
pub use webhook::*;

#[derive(Debug)]
pub enum APIEndpoint {
    GetMe,
    CreateInvoice,
    DeleteInvoice,
    CreateCheck,
    DeleteCheck,
    Transfer,
    GetInvoices,
    GetChecks,
    GetTransfers,
    GetBalance,
    GetExchangeRates,
    GetCurrencies,
    GetStats,
}

impl APIEndpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            APIEndpoint::GetMe => "getMe",
            APIEndpoint::CreateInvoice => "createInvoice",
            APIEndpoint::DeleteInvoice => "deleteInvoice",
            APIEndpoint::CreateCheck => "createCheck",
            APIEndpoint::DeleteCheck => "deleteCheck",
            APIEndpoint::Transfer => "transfer",
            APIEndpoint::GetInvoices => "getInvoices",
            APIEndpoint::GetChecks => "getChecks",
            APIEndpoint::GetTransfers => "getTransfers",
            APIEndpoint::GetBalance => "getBalance",
            APIEndpoint::GetExchangeRates => "getExchangeRates",
            APIEndpoint::GetCurrencies => "getCurrencies",
            APIEndpoint::GetStats => "getStats",
        }
    }
}

pub enum Method {
    POST,
    GET,
    DELETE,
}

pub struct APIMethod {
    pub endpoint: APIEndpoint,
    pub method: Method,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum PayButtonName {
    #[serde(rename = "viewItem")]
    ViewItem,
    #[serde(rename = "openChannel")]
    OpenChannel,
    #[serde(rename = "openBot")]
    OpenBot,
    #[serde(rename = "callback")]
    Callback,
}
