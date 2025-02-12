mod config;
mod handler;

pub use config::{WebhookHandlerConfig, WebhookHandlerConfigBuilder};
pub use handler::WebhookHandler;

use crate::client::CryptoBot;

impl CryptoBot {
    /// Creates a new webhook handler with the given config
    ///
    /// # Example
    /// ```no_run
    /// use crypto_pay_api::prelude::*;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), CryptoBotError> {
    ///     let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build().unwrap();
    ///
    ///     let config = WebhookHandlerConfigBuilder::new()
    ///         .expiration_time(Duration::from_secs(60 * 10))
    ///         .build();
    ///
    ///     let webhook_handler = client.webhook_handler(config);
    ///     Ok(())
    /// }
    /// ```
    pub fn webhook_handler(&self, config: WebhookHandlerConfig) -> WebhookHandler {
        WebhookHandler::with_config(&self.api_token, config)
    }
}
