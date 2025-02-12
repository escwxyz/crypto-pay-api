use std::time::Duration;

use crate::client::DEFAULT_WEBHOOK_EXPIRATION_TIME;

#[derive(Debug, Default)]
pub struct WebhookHandlerConfig {
    pub expiration_time: Option<Duration>,
}

pub struct WebhookHandlerConfigBuilder {
    pub config: WebhookHandlerConfig,
}

impl WebhookHandlerConfigBuilder {
    /// Creates a new webhook handler config with default expiration time
    ///
    /// # Default Settings
    /// * Expiration time: 10 minutes
    pub fn new() -> Self {
        Self {
            config: WebhookHandlerConfig {
                expiration_time: Some(Duration::from_secs(DEFAULT_WEBHOOK_EXPIRATION_TIME)),
            },
        }
    }
    /// Sets the expiration time for the webhook handler
    pub fn expiration_time(mut self, duration: Duration) -> Self {
        self.config.expiration_time = Some(duration);
        self
    }
    /// Disables the expiration time for the webhook handler
    pub fn disable_expiration(mut self) -> Self {
        self.config.expiration_time = None;
        self
    }
    /// Builds the webhook handler config
    pub fn build(self) -> WebhookHandlerConfig {
        self.config
    }
}

impl Default for WebhookHandlerConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_webhook_handler_config_builder() {
        let config = WebhookHandlerConfigBuilder::new()
            .expiration_time(Duration::from_secs(1000))
            .build();

        assert_eq!(config.expiration_time, Some(Duration::from_secs(1000)));
    }

    #[test]
    fn test_webhook_handler_config_builder_disable_expiration() {
        let config = WebhookHandlerConfigBuilder::new().disable_expiration().build();

        assert_eq!(config.expiration_time, None);
    }

    #[test]
    fn test_webhook_handler_config_builder_default() {
        let config = WebhookHandlerConfigBuilder::default().build();

        assert_eq!(config.expiration_time, Some(Duration::from_secs(600)));
    }
}
