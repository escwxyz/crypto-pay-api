use serde::{Deserialize, Serialize};

use super::Invoice;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType {
    #[serde(rename = "invoice_paid")]
    InvoicePaid,
}
#[derive(Debug, Deserialize)]
pub struct WebhookUpdate {
    pub update_id: i64,
    pub update_type: UpdateType,
    pub request_date: String,
    pub payload: WebhookPayload,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebhookPayload {
    InvoicePaid(Invoice),
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
