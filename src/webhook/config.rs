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

    pub fn expiration_time(mut self, duration: Duration) -> Self {
        self.config.expiration_time = Some(duration);
        self
    }

    pub fn disable_expiration(mut self) -> Self {
        self.config.expiration_time = None;
        self
    }

    pub fn build(self) -> WebhookHandlerConfig {
        self.config
    }
}
