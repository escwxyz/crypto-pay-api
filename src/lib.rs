pub mod balance;
pub mod check;
pub mod client;
pub mod error;
pub mod exchange;
pub mod invoice;
pub mod model;
pub mod stats;
pub mod traits;
pub mod transfer;
pub mod webhook;

mod test_utils;

pub use client::CryptoBot;
pub use error::CryptoBotError;

pub use webhook::{WebhookHandler, WebhookResponse, WebhookUpdate};

#[cfg(feature = "axum-webhook")]
pub use webhook::axum::webhook_middleware;
