use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, delete},
    Json, Router,
};
use std::sync::Arc;

use crate::AppState;
use probook_core::db::repository;
use probook_core::models::*;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/payments/by-invoice/{invoiceId}", get(get_payments_by_invoice))
        .route("/payments", post(create_payment))
        .route("/payments/{id}", delete(delete_payment))
}

async fn get_payments_by_invoice(
    State(state): State<Arc<AppState>>,
    Path(invoice_id): Path<String>,
) -> Result<Json<Vec<Payment>>, (StatusCode, String)> {
    let payments = repository::get_payments_by_invoice(&state.pool, &invoice_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(payments))
}

async fn create_payment(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreatePaymentInput>,
) -> Result<Json<Payment>, (StatusCode, String)> {
    let payment = repository::create_payment(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(payment))
}

async fn delete_payment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_payment(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}
