use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::pin::Pin;
use std::{future::Future, time::Duration};

use crate::{
    client::DEFAULT_WEBHOOK_EXPIRATION_TIME,
    error::{CryptoBotError, WebhookErrorKind},
    models::{WebhookResponse, WebhookUpdate},
};

pub type WebhookHandlerFn = Box<
    dyn Fn(WebhookUpdate) -> Pin<Box<dyn Future<Output = Result<(), CryptoBotError>> + Send>>
        + Send
        + Sync,
>;

pub struct WebhookHandler {
    api_token: String,
    webhook_expiration_time: Option<Duration>,
    update_handler: Option<WebhookHandlerFn>,
}

impl WebhookHandler {
    pub(crate) fn new(api_token: &str) -> Self {
        Self {
            api_token: api_token.to_string(),
            webhook_expiration_time: Some(Duration::from_secs(DEFAULT_WEBHOOK_EXPIRATION_TIME)),
            update_handler: None,
        }
    }

    pub(crate) fn with_expiration_time(mut self, duration: Duration) -> Self {
        self.webhook_expiration_time = Some(duration);
        self
    }

    /// Parses a webhook update from JSON string
    ///
    /// # Arguments
    /// * `json` - The JSON string to parse
    ///
    /// # Returns
    /// * `Ok(WebhookUpdate)` - The parsed webhook update
    /// * `Err(CryptoBotError)` - If the JSON is invalid or doesn't match the expected format
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::WebhookHandler;
    ///
    /// let json = r#"{
    ///        "update_id": 1,
    ///        "update_type": "invoice_paid",
    ///        "request_date": "2024-02-02T12:11:02Z",
    ///        "payload": {
    ///            "invoice_id": 528890,
    ///            "hash": "IVDoTcNBYEfk",
    ///            "currency_type": "crypto",
    ///            "asset": "TON",
    ///            "amount": "10.5",
    ///            "pay_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
    ///            "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
    ///            "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVDoTcNBYEfk",
    ///            "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVDoTcNBYEfk",
    ///            "description": "Test invoice",
    ///            "status": "paid",
    ///            "created_at": "2025-02-08T12:11:01.341Z",
    ///            "allow_comments": true,
    ///            "allow_anonymous": true
    ///        }
    /// }"#;
    ///
    /// let result = WebhookHandler::parse_update(json);
    /// assert!(result.is_ok());
    /// ```
    pub fn parse_update(json: &str) -> Result<WebhookUpdate, CryptoBotError> {
        serde_json::from_str(json).map_err(|e| CryptoBotError::WebhookError {
            kind: WebhookErrorKind::InvalidPayload,
            message: e.to_string(),
        })
    }

    /// Verifies the signature of a webhook request
    ///
    /// The signature is created by the Crypto Bot API using HMAC-SHA-256
    /// with the API token as the key and the request body as the message.
    ///
    /// # Arguments
    /// * `body` - The raw request body
    /// * `signature` - The signature from the 'crypto-pay-api-signature' header
    ///
    /// # Returns
    /// * `true` if the signature is valid
    /// * `false` if the signature is invalid or malformed
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// let client = CryptoBot::new("your_api_token", None);
    /// let handler = client.webhook_handler();
    /// let body = r#"{"update_id": 1, "update_type": "invoice_paid"}"#;
    /// let signature = "1234567890abcdef"; // The actual signature from the request header
    ///
    /// if handler.verify_signature(body, signature) {
    ///     println!("Signature is valid");
    /// } else {
    ///     println!("Invalid signature");
    /// }
    /// ```
    pub fn verify_signature(&self, body: &str, signature: &str) -> bool {
        let secret = Sha256::digest(self.api_token.as_bytes());
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&secret).expect("HMAC can take key of any size");

        mac.update(body.as_bytes());

        if let Ok(hex_signature) = hex::decode(signature) {
            mac.verify_slice(&hex_signature).is_ok()
        } else {
            false
        }
    }

    /// Handles a webhook update from Crypto Bot API
    ///
    /// This method:
    /// 1. Parses the webhook update from JSON
    /// 2. Validates the request date
    /// 3. Checks if the request has expired
    /// 4. Calls the registered update handler if one exists
    ///
    /// # Arguments
    /// * `body` - The raw webhook request body as JSON string
    ///
    /// # Returns
    /// * `Ok(WebhookResponse)` - If the update was handled successfully
    /// * `Err(CryptoBotError)` - If any validation fails or the handler returns an error
    ///
    /// # Errors
    /// * `WebhookErrorKind::InvalidPayload` - If the JSON is invalid or missing required fields
    /// * `WebhookErrorKind::Expired` - If the request is older than the expiration time
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// async fn handle_webhook(body: String, signature: &str, handler: &WebhookHandler) {
    ///     // First verify the webhook signature
    ///     if !handler.verify_signature(&body, signature) {
    ///         eprintln!("Invalid signature");
    ///         return;
    ///     }
    ///     
    ///     // Then handle the update
    ///     match handler.handle_update(&body).await {
    ///         Ok(_) => println!("Webhook handled successfully"),
    ///         Err(e) => match e {
    ///             CryptoBotError::WebhookError { kind: WebhookErrorKind::Expired, .. } => {
    ///                 eprintln!("Webhook request too old");
    ///             }
    ///             _ => eprintln!("Error handling webhook: {}", e),
    ///         },
    ///     }
    /// }
    /// ```
    pub async fn handle_update(&self, body: &str) -> Result<WebhookResponse, CryptoBotError> {
        let update: WebhookUpdate = Self::parse_update(body)?;

        // Verify request date
        let request_date = DateTime::parse_from_rfc3339(&update.request_date).map_err(|_| {
            CryptoBotError::WebhookError {
                kind: WebhookErrorKind::InvalidPayload,
                message: "Invalid request date".to_string(),
            }
        })?;

        let age = Utc::now().signed_duration_since(request_date.with_timezone(&Utc));

        let webhook_expiration_time = self
            .webhook_expiration_time
            .unwrap_or(Duration::from_secs(DEFAULT_WEBHOOK_EXPIRATION_TIME))
            .as_secs();

        let webhook_expiration = chrono::Duration::seconds(webhook_expiration_time as i64);

        if age > webhook_expiration {
            return Err(CryptoBotError::WebhookError {
                kind: WebhookErrorKind::Expired,
                message: "Webhook request too old".to_string(),
            });
        }

        if let Some(handler) = &self.update_handler {
            handler(update).await?;
        }

        Ok(WebhookResponse::ok())
    }

    /// Registers a handler function for webhook updates
    ///
    /// The handler function will be called for each webhook update received through
    /// `handle_update`. The function should process the update and return a Result
    /// indicating success or failure.
    ///
    /// # Arguments
    /// * `handler` - An async function that takes a `WebhookUpdate` and returns a `Result<(), CryptoBotError>`
    ///
    /// # Type Parameters
    /// * `F` - The handler function type
    /// * `Fut` - The future type returned by the handler
    ///
    /// # Requirements
    /// The handler function must:
    /// * Be `Send` + `Sync` + 'static
    /// * Return a Future that is `Send` + 'static
    /// * The Future must resolve to `Result<(), CryptoBotError>`
    ///
    /// # Example
    /// ```
    /// use crypto_pay_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = CryptoBot::new("YOUR_API_TOKEN", None);
    ///     let mut handler = client.webhook_handler();
    ///
    ///     handler.on_update(|update| async move {
    ///         match (update.update_type, update.payload) {
    ///             (UpdateType::InvoicePaid, WebhookPayload::InvoicePaid(invoice)) => {
    ///                 println!("Payment received!");
    ///                 println!("Amount: {} {}", invoice.amount, invoice.asset.unwrap());
    ///                 println!("Status: {}", invoice.status);
    ///                 
    ///                 // Process the payment...
    ///             }
    ///         }
    ///         Ok(())
    ///     });
    ///
    ///     // Now ready to handle webhook updates
    /// }
    /// ```
    pub fn on_update<F, Fut>(&mut self, handler: F)
    where
        F: Fn(WebhookUpdate) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), CryptoBotError>> + Send + 'static,
    {
        self.update_handler = Some(Box::new(move |update| Box::pin(handler(update))));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InvoiceStatus, UpdateType, WebhookPayload};
    use chrono::Utc;
    use serde_json::json;

    use std::{sync::Arc, time::Duration};
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_webhook_handler() {
        let mut handler = WebhookHandler::new("test_token");

        let received = Arc::new(Mutex::new(None));
        let received_clone = received.clone();

        handler.on_update(move |update| {
            let received = received_clone.clone();
            async move {
                let mut guard = received.lock().await;
                *guard = Some(update);
                Ok(())
            }
        });

        let json = json!({
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": Utc::now().to_rfc3339(),
            "payload": {
                "invoice_id": 528890,
                "hash": "IVDoTcNBYEfk",
                "currency_type": "crypto",
                "asset": "TON",
                "amount": "10.5",
                "pay_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVDoTcNBYEfk",
                "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVDoTcNBYEfk",
                "description": "Test invoice",
                "status": "paid",
                "created_at": "2025-02-08T12:11:01.341Z",
                "allow_comments": true,
                "allow_anonymous": true
            }
        }).to_string();

        let result = handler.handle_update(&json).await;
        assert!(result.is_ok());

        let update = received
            .lock()
            .await
            .take()
            .expect("Should have received update");
        assert_eq!(update.update_type, UpdateType::InvoicePaid);
        match update.payload {
            WebhookPayload::InvoicePaid(invoice) => {
                assert_eq!(invoice.invoice_id, 528890);
                assert_eq!(invoice.status, InvoiceStatus::Paid);
            }
        }
    }

    #[tokio::test]
    async fn test_default_webhook_expiration() {
        let handler = WebhookHandler::new("test_token");

        let date = (Utc::now() - chrono::Duration::minutes(3)).to_rfc3339();

        let json = json!({
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": date,
            "payload":  {
                    "invoice_id": 528890,
                    "hash": "IVDoTcNBYEfk",
                    "currency_type": "crypto",
                    "asset": "TON",
                    "amount": "10.5",
                    "pay_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                    "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                    "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVDoTcNBYEfk",
                    "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVDoTcNBYEfk",
                    "description": "Test invoice",
                    "status": "paid",
                    "created_at": "2025-02-08T12:11:01.341Z",
                    "allow_comments": true,
                    "allow_anonymous": true
            }
        }).to_string();

        let result = handler.handle_update(&json).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_custom_webhook_expiration() {
        let handler =
            WebhookHandler::new("test_token").with_expiration_time(Duration::from_secs(60));

        let old_date = (Utc::now() - chrono::Duration::minutes(2)).to_rfc3339();

        let json = json!({
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": old_date,
            "payload": {
                    "invoice_id": 528890,
                    "hash": "IVDoTcNBYEfk",
                    "currency_type": "crypto",
                    "asset": "TON",
                    "amount": "10.5",
                    "pay_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                    "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                    "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVDoTcNBYEfk",
                    "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVDoTcNBYEfk",
                    "description": "Test invoice",
                    "status": "paid",
                    "created_at": "2025-02-08T12:11:01.341Z",
                    "allow_comments": true,
                    "allow_anonymous": true
                }
        })
        .to_string();

        let result = handler.handle_update(&json).await;
        assert!(matches!(
            result,
            Err(CryptoBotError::WebhookError {
                kind: WebhookErrorKind::Expired,
                ..
            })
        ));
    }

    #[test]
    fn test_webhook_signature_verification() {
        let handler = WebhookHandler::new("test_token");
        let body = json!({
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": "2024-01-01T12:00:00Z",
            "payload": {
                "invoice_id": 528890,
                "hash": "IVDoTcNBYEfk",
                "status": "paid",
                // ... other invoice fields ...
            }
        })
        .to_string();

        // Generate a valid signature
        let secret = Sha256::digest(b"test_token");
        let mut mac = Hmac::<Sha256>::new_from_slice(&secret).unwrap();
        mac.update(body.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        assert!(handler.verify_signature(&body, &signature));
        assert!(!handler.verify_signature(&body, "invalid_signature"));
    }

    #[test]
    fn test_parse_webhook_update() {
        let json = json!({
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": "2024-02-02T12:11:02Z",
            "payload": {
                "invoice_id": 528890,
                "hash": "IVDoTcNBYEfk",
                "currency_type": "crypto",
                "asset": "TON",
                "amount": "10.5",
                "pay_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                "bot_invoice_url": "https://t.me/CryptoTestnetBot?start=IVDoTcNBYEfk",
                "mini_app_invoice_url": "https://t.me/CryptoTestnetBot/app?startapp=invoice-IVDoTcNBYEfk",
                "web_app_invoice_url": "https://testnet-app.send.tg/invoices/IVDoTcNBYEfk",
                "description": "Test invoice",
                "status": "paid",
                "created_at": "2025-02-08T12:11:01.341Z",
                "allow_comments": true,
                "allow_anonymous": true
            }
        });

        let result = WebhookHandler::parse_update(&json.to_string());
        assert!(result.is_ok());

        let update = result.unwrap();
        assert_eq!(update.update_id, 1);
        assert_eq!(update.update_type, UpdateType::InvoicePaid);
        assert_eq!(update.request_date, "2024-02-02T12:11:02Z");

        match update.payload {
            WebhookPayload::InvoicePaid(invoice) => {
                assert_eq!(invoice.invoice_id, 528890);
                assert_eq!(invoice.status, InvoiceStatus::Paid);
            }
        }
    }

    #[test]
    fn test_parse_invalid_webhook_update() {
        let invalid_json = r#"{"invalid": "json"}"#;

        let result = WebhookHandler::parse_update(invalid_json);
        assert!(matches!(
            result,
            Err(CryptoBotError::WebhookError {
                kind: WebhookErrorKind::InvalidPayload,
                ..
            })
        ));
    }
}
