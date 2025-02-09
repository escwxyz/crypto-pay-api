mod api;
mod client;
mod error;
mod models;
mod utils;
mod validation;
mod webhook;

pub mod prelude {
    pub use rust_decimal::Decimal;

    pub use crate::models::*;

    pub use crate::api::*;

    pub use crate::client::CryptoBot;

    pub use crate::webhook::*;

    #[cfg(feature = "axum-webhook")]
    pub use crate::webhook::axum::webhook_middleware;
}
pub use prelude::*;
