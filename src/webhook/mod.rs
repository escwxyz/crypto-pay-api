mod handler;
use std::time::Duration;

pub use handler::WebhookHandler;

use crate::CryptoBot;

impl CryptoBot {
    /// Creates a new webhook handler with default settings
    ///
    /// # Default Settings
    /// * Expiration time: 600 seconds (10 minutes)
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// let client = CryptoBot::new("YOUR_API_TOKEN", None);
    /// let webhook_handler = client.webhook_handler();
    /// ```
    pub fn webhook_handler(&self) -> WebhookHandler {
        WebhookHandler::new(&self.api_token)
    }

    /// Creates a new webhook handler with custom expiration time
    ///
    /// # Arguments
    /// * `expiration_time` - The duration after which webhook requests are considered expired
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    /// use std::time::Duration;
    ///
    /// let client = CryptoBot::new("YOUR_API_TOKEN", None);
    /// let webhook_handler = client.webhook_handler_with_expiration(
    ///     Duration::from_secs(300)  // 5 minutes
    /// );
    /// ```
    pub fn webhook_handler_with_expiration(&self, expiration_time: Duration) -> WebhookHandler {
        WebhookHandler::new(&self.api_token).with_expiration_time(expiration_time)
    }
}
