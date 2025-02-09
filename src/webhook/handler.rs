use chrono::{DateTime, Utc};
use std::future::Future;
use std::pin::Pin;

use crate::{error::CryptoBotError, error::WebhookErrorKind, CryptoBot};

use super::{WebhookInvoicePaid, WebhookPayload, WebhookResponse, WebhookUpdate};

pub type WebhookHandlerFn = Box<
    dyn Fn(WebhookInvoicePaid) -> Pin<Box<dyn Future<Output = Result<(), CryptoBotError>> + Send>>
        + Send
        + Sync,
>;

pub struct WebhookHandler {
    pub crypto_bot: CryptoBot,
    invoice_paid_handler: Option<WebhookHandlerFn>,
}

impl WebhookHandler {
    pub fn new(crypto_bot: CryptoBot) -> Self {
        Self {
            crypto_bot,
            invoice_paid_handler: None,
        }
    }

    pub fn on_invoice_paid<F, Fut>(&mut self, handler: F)
    where
        F: Fn(WebhookInvoicePaid) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), CryptoBotError>> + Send + 'static,
    {
        self.invoice_paid_handler = Some(Box::new(move |update| Box::pin(handler(update))));
    }

    pub async fn handle_update(&self, body: &str) -> Result<WebhookResponse, CryptoBotError> {
        let update: WebhookUpdate = CryptoBot::parse_webhook_update(body)?;

        // Verify request date
        let request_date = DateTime::parse_from_rfc3339(&update.request_date).map_err(|_| {
            CryptoBotError::WebhookError {
                kind: WebhookErrorKind::InvalidPayload,
                message: "Invalid request date".to_string(),
            }
        })?;

        let age = Utc::now().signed_duration_since(request_date.with_timezone(&Utc));
        if age.num_minutes() > 5 {
            return Err(CryptoBotError::WebhookError {
                kind: WebhookErrorKind::Expired,
                message: "Webhook request too old".to_string(),
            });
        }

        // Handle update based on payload
        match update.payload {
            WebhookPayload::InvoicePaid(invoice_paid) => {
                if let Some(handler) = &self.invoice_paid_handler {
                    handler(invoice_paid).await?;
                }
            }
        }

        Ok(WebhookResponse::ok())
    }
}
