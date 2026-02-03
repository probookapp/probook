use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

use crate::AppState;
use probook_core::db::repository;
use probook_core::models::*;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/invoices", get(get_all_invoices).post(create_invoice))
        .route("/invoices/{id}", get(get_invoice_by_id).put(update_invoice).delete(delete_invoice))
        .route("/invoices/batch-delete", post(batch_delete_invoices))
        .route("/invoices/{id}/issue", post(issue_invoice))
        .route("/invoices/{id}/mark-paid", post(mark_invoice_paid))
        .route("/invoices/{id}/verify-integrity", get(verify_invoice_integrity))
        .route("/invoices/{id}/duplicate", post(duplicate_invoice))
        .route("/invoices/{id}/convert-to-delivery-note", post(convert_invoice_to_delivery_note))
        .route("/invoices/from-delivery-notes", post(create_invoice_from_delivery_notes))
}

async fn get_all_invoices(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Invoice>>, (StatusCode, String)> {
    let invoices = repository::get_all_invoices(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoices))
}

async fn get_invoice_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::get_invoice_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}

async fn create_invoice(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateInvoiceInput>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::create_invoice(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}

async fn update_invoice(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateInvoiceInput>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    input.id = id;
    let invoice = repository::update_invoice(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}

async fn delete_invoice(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_invoice(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn batch_delete_invoices(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = repository::batch_delete_invoices(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(count))
}

async fn issue_invoice(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::issue_invoice(&state.pool, &id, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}

async fn mark_invoice_paid(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::mark_invoice_paid(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}

async fn verify_invoice_integrity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<bool>, (StatusCode, String)> {
    let valid = repository::verify_invoice_integrity(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(valid))
}

async fn duplicate_invoice(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::duplicate_invoice(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}

async fn convert_invoice_to_delivery_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeliveryNote>, (StatusCode, String)> {
    let delivery_note = repository::convert_invoice_to_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(delivery_note))
}

async fn create_invoice_from_delivery_notes(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::create_invoice_from_delivery_notes(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}
