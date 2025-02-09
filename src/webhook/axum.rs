use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use super::handler::WebhookHandler;

pub async fn webhook_middleware(
    State(handler): State<Arc<WebhookHandler>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    // Get signature from header
    let signature = parts
        .headers
        .get("crypto-pay-api-signature")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Get body as string
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let body_str = String::from_utf8(body_bytes.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Verify signature
    if !handler
        .crypto_bot
        .verify_webhook_signature(&body_str, signature)
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Handle webhook
    handler
        .handle_update(&body_str)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Reconstruct request
    let req = Request::from_parts(parts, Body::from(body_str));
    Ok(next.run(req).await)
}

#[cfg(all(test, feature = "axum-webhook"))]
mod tests {
    use crate::CryptoBot;

    use super::*;
    use axum::{body::Body, http::Request, Router};
    use hmac::{Hmac, Mac};
    use sha2::{Digest, Sha256};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_webhook_middleware() {
        let client = CryptoBot::new("test_token", None);
        let handler = Arc::new(WebhookHandler::new(client));

        let app = Router::new()
            .route("/webhook", axum::routing::post(|| async { "OK" }))
            .layer(axum::middleware::from_fn_with_state(
                handler.clone(),
                webhook_middleware,
            ));

        let body =
            r#"{"update_id":1,"update_type":"invoice_paid","request_date":"2024-01-01T12:00:00Z"}"#;

        // Generate valid signature
        let secret = Sha256::digest(b"test_token");
        let mut mac = Hmac::<Sha256>::new_from_slice(&secret).unwrap();
        mac.update(body.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .header("crypto-pay-api-signature", signature)
            .body(Body::from(body))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
