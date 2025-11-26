mod config;
mod handler;

pub use config::{WebhookHandlerConfig, WebhookHandlerConfigBuilder};
pub use handler::WebhookHandler;

use crate::client::CryptoBot;

impl CryptoBot {
    /// Creates a new webhook handler builder
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
    ///     let webhook_handler = client.webhook_handler()
    ///         .expiration_time(Duration::from_secs(60 * 10))
    ///         .build();
    ///     Ok(())
    /// }
    /// ```
    pub fn webhook_handler(&self) -> WebhookHandlerConfigBuilder<'_> {
        WebhookHandlerConfigBuilder::new_with_client(&self.api_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_webhook_handler_creation() {
        let client = CryptoBot::test_client();

        // Test with default config
        let handler = client.webhook_handler().build();

        assert_eq!(handler.api_token, client.api_token);
        assert_eq!(handler.config.expiration_time, Some(Duration::from_secs(600)));

        // Test with custom config
        let handler = client
            .webhook_handler()
            .expiration_time(Duration::from_secs(300))
            .build();

        assert_eq!(handler.api_token, client.api_token);
        assert_eq!(handler.config.expiration_time, Some(Duration::from_secs(300)));
    }
}
