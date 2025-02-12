use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::post,
    Router,
};
use crypto_pay_api::prelude::*;
use std::sync::Arc;

async fn webhook_middleware(
    State(handler): State<Arc<WebhookHandler>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    let signature = parts
        .headers
        .get("crypto-pay-api-signature")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let body_str = String::from_utf8(body_bytes.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?;

    if !handler.verify_signature(&body_str, signature) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    handler
        .handle_update(&body_str)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let req = Request::from_parts(parts, Body::from(body_str));
    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() {
    let client = CryptoBot::builder().api_token("your_token").build().unwrap();

    let mut webhook_handler = client.webhook_handler(WebhookHandlerConfigBuilder::new().build());

    // Register handlers
    webhook_handler.on_update(|update| async move {
        println!("Invoice paid: {:?}", update.payload);
        Ok(())
    });

    let handler = Arc::new(webhook_handler);

    let app = Router::new()
        .route("/webhook", post(|| async { "OK" }))
        .with_state(handler.clone())
        .layer(middleware::from_fn_with_state(handler, webhook_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
