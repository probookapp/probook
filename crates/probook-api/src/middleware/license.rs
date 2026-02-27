use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use probook_core::services::licensing::engine as licensing_engine;

/// Middleware that blocks write operations (POST/PUT/DELETE) when the license
/// does not allow writes. Returns HTTP 402 Payment Required.
/// GET and OPTIONS requests always pass through.
pub async fn license_middleware(
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    let method = request.method().clone();
    if method != Method::GET && method != Method::OPTIONS {
        if !licensing_engine::is_write_allowed() {
            return Err((
                StatusCode::PAYMENT_REQUIRED,
                "License expired \u{2014} write access is disabled. Please renew your license.",
            )
                .into_response());
        }
    }
    Ok(next.run(request).await)
}
