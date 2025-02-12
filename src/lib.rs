//! crypto-pay-api
//!
//! A Rust client library for interacting with the CryptoPay API.
//!
//! # Example with tokio
//!
//! ```no_run
//! use crypto_pay_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), CryptoBotError> {
//!     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build()?;
//!     let params = CreateInvoiceParamsBuilder::new().amount(dec!(10.0)).asset(CryptoCurrencyCode::Ton).build(&client).await?;
//!     let invoice = client.create_invoice(&params).await?;
//!     println!("Invoice created: {}", invoice.invoice_id);
//!     Ok(())
//! }
//! ```
//!
//! # Webhook Handling with axum
//!
//! ```no_run
//! use crypto_pay_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), CryptoBotError> {
//!     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build()?;
//!     let mut handler = client.webhook_handler(WebhookHandlerConfigBuilder::new().build());
//!     handler.on_update(|update| async move {
//!         println!("Invoice paid: {:?}", update.payload);
//!         Ok(())
//!     });
//!     Ok(())
//! }
//! ```
//!
//! For issues and contributions, please refer to the [GitHub repository](https://github.com/escwxyz/crypto-pay-api).

mod api;
mod client;
mod error;
mod models;
mod utils;
mod validation;
mod webhook;

pub mod prelude {
    // Third-party crates re-exports
    pub use crate::utils::types::*;

    // Local crates re-exports
    pub use crate::api::*;
    pub use crate::client::CryptoBot;
    pub use crate::error::*;
    pub use crate::models::*;
    pub use crate::webhook::*;
}
