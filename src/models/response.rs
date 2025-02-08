use serde::Deserialize;

use super::Invoice;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error: Option<String>,
    pub error_code: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GetMeResponse {
    /// Unique ID of the application.
    pub app_id: i64,
    /// Name of the application.
    pub name: String,
    /// Username of the payment processing bot.
    pub payment_processing_bot_username: String,
    /// Optional. Webhook endpoint for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_endpoint: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetInvoicesResponse {
    pub items: Vec<Invoice>,
}
