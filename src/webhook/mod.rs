#[cfg(feature = "axum-webhook")]
pub mod axum;

mod handler;
pub use handler::WebhookHandler;

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    error::{CryptoBotError, WebhookErrorKind},
    models::Invoice,
    CryptoBot,
};

#[derive(Debug, Deserialize)]
pub struct WebhookUpdate {
    pub update_id: i64,
    pub update_type: String,
    pub request_date: String,
    pub payload: WebhookPayload,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WebhookPayload {
    InvoicePaid(WebhookInvoicePaid),
}

#[derive(Debug, Deserialize)]
pub struct WebhookInvoicePaid {
    pub invoice: Invoice,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub ok: bool,
}

impl WebhookResponse {
    pub fn ok() -> Self {
        Self { ok: true }
    }
}

impl CryptoBot {
    pub fn parse_webhook_update(json: &str) -> Result<WebhookUpdate, CryptoBotError> {
        serde_json::from_str(json).map_err(|e| CryptoBotError::WebhookError {
            kind: WebhookErrorKind::InvalidPayload,
            message: e.to_string(),
        })
    }

    pub fn verify_webhook_signature(&self, body: &str, signature: &str) -> bool {
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
}
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::InvoiceStatus;

    use super::*;
    use chrono::Utc;
    use serde_json::json;
    use tokio::sync::Mutex;

    fn create_test_client() -> CryptoBot {
        CryptoBot::new("test_token", None)
    }

    #[test]
    fn test_webhook_signature_verification() {
        let client = create_test_client();
        let body =
            r#"{"update_id":1,"update_type":"invoice_paid","request_date":"2024-01-01T12:00:00Z"}"#;

        // Generate a valid signature
        let secret = Sha256::digest(b"test_token");
        let mut mac = Hmac::<Sha256>::new_from_slice(&secret).unwrap();
        mac.update(body.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // Test valid signature
        assert!(client.verify_webhook_signature(body, &signature));

        // Test invalid signature
        assert!(!client.verify_webhook_signature(body, "invalid_signature"));
        assert!(!client.verify_webhook_signature(body, "deadbeef"));
    }

    #[test]
    fn test_parse_webhook_update() {
        create_test_client();
        let json = json!({
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": "2024-01-01T12:00:00Z",
            "payload": {
                "invoice": {
                    "invoice_id": 123,
                    "status": "paid",
                    "hash": "test_hash",
                    "asset": "TON",
                    "amount": "10.5",
                    "pay_url": "https://example.com",
                    "created_at": "2024-01-01T12:00:00Z"
                }
            }
        })
        .to_string();

        let result = CryptoBot::parse_webhook_update(&json);

        println!("result: {:?}", result);

        assert!(result.is_ok());

        let update = result.unwrap();
        assert_eq!(update.update_id, 1);
        assert_eq!(update.update_type, "invoice_paid");
        assert_eq!(update.request_date, "2024-01-01T12:00:00Z");

        match update.payload {
            WebhookPayload::InvoicePaid(paid) => {
                assert_eq!(paid.invoice.invoice_id, 123);
                assert_eq!(paid.invoice.status, InvoiceStatus::Paid);
            }
        }
    }

    #[test]
    fn test_parse_invalid_webhook_update() {
        let _ = create_test_client();
        let invalid_json = r#"{"invalid": "json"}"#;

        let result = CryptoBot::parse_webhook_update(invalid_json);
        assert!(matches!(
            result,
            Err(CryptoBotError::WebhookError {
                kind: WebhookErrorKind::InvalidPayload,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn test_webhook_handler() {
        let client = create_test_client();
        let mut handler = WebhookHandler::new(client);

        let received = Arc::new(Mutex::new(None));
        let received_clone = received.clone();

        handler.on_invoice_paid(move |update| {
            let received = received_clone.clone();
            async move {
                let mut guard = received.lock().await;
                *guard = Some(update.invoice.invoice_id);
                Ok(())
            }
        });

        let json = json!({
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": Utc::now().to_rfc3339(),
            "payload": {
                "invoice": {
                    "invoice_id": 123,
                    "status": "paid",
                    "hash": "test_hash",
                    "asset": "TON",
                    "amount": "10.5",
                    "pay_url": "https://example.com",
                    "created_at": "2024-01-01T12:00:00Z"
                }
            }
        })
        .to_string();

        let result = handler.handle_update(&json).await;
        assert!(result.is_ok());

        let received_id = *received.lock().await;
        assert_eq!(received_id, Some(123));
    }

    #[tokio::test]
    async fn test_expired_webhook() {
        let client = CryptoBot::new("test_token", None);
        let handler = WebhookHandler::new(client);

        let old_date = (Utc::now() - chrono::Duration::minutes(10)).to_rfc3339();

        let json = format!(
            r#"{{
            "update_id": 1,
            "update_type": "invoice_paid",
            "request_date": "{}",
            "payload": {{
                "invoice": {{
                    "invoice_id": 123,
                    "status": "paid",
                    "hash": "test_hash",
                    "asset": "TON",
                    "amount": "10.5",
                    "pay_url": "https://example.com",
                    "created_at": "2024-01-01T12:00:00Z"
                }}
            }}
        }}"#,
            old_date
        );

        let result = handler.handle_update(&json).await;
        assert!(matches!(
            result,
            Err(CryptoBotError::WebhookError {
                kind: WebhookErrorKind::Expired,
                ..
            })
        ));
    }
}
