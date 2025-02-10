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
pub use prelude::*;
