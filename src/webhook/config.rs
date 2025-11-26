use std::time::Duration;

use crate::client::DEFAULT_WEBHOOK_EXPIRATION_TIME;

#[derive(Debug, Default)]
pub struct WebhookHandlerConfig {
    pub expiration_time: Option<Duration>,
}

pub struct WebhookHandlerConfigBuilder<'a> {
    api_token: Option<&'a str>,
    config: WebhookHandlerConfig,
}

impl<'a> WebhookHandlerConfigBuilder<'a> {
    /// Creates a new webhook handler config builder with default expiration time
    ///
    /// # Default Settings
    /// * Expiration time: 10 minutes
    pub fn new() -> Self {
        Self {
            api_token: None,
            config: WebhookHandlerConfig {
                expiration_time: Some(Duration::from_secs(DEFAULT_WEBHOOK_EXPIRATION_TIME)),
            },
        }
    }

    /// Creates a new webhook handler config builder with client reference
    pub(crate) fn new_with_client(api_token: &'a str) -> Self {
        Self {
            api_token: Some(api_token),
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

    /// Builds the webhook handler config (for backward compatibility)
    pub fn build_config(self) -> WebhookHandlerConfig {
        self.config
    }

    /// Builds the webhook handler (requires client reference)
    pub fn build(self) -> crate::webhook::handler::WebhookHandler {
        let api_token = self
            .api_token
            .expect("WebhookHandlerConfigBuilder must be created via client.webhook_handler()");
        crate::webhook::handler::WebhookHandler::with_config(api_token, self.config)
    }
}

impl<'a> Default for WebhookHandlerConfigBuilder<'a> {
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
        let builder = WebhookHandlerConfigBuilder::new().expiration_time(Duration::from_secs(1000));

        assert_eq!(builder.config.expiration_time, Some(Duration::from_secs(1000)));
    }

    #[test]
    fn test_webhook_handler_config_builder_disable_expiration() {
        let builder = WebhookHandlerConfigBuilder::new().disable_expiration();

        assert_eq!(builder.config.expiration_time, None);
    }

    #[test]
    fn test_webhook_handler_config_builder_default() {
        let builder = WebhookHandlerConfigBuilder::default();

        assert_eq!(builder.config.expiration_time, Some(Duration::from_secs(600)));
    }
}
