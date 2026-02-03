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
        .route("/quotes", get(get_all_quotes).post(create_quote))
        .route("/quotes/{id}", get(get_quote_by_id).put(update_quote).delete(delete_quote))
        .route("/quotes/batch-delete", post(batch_delete_quotes))
        .route("/quotes/{id}/convert-to-invoice", post(convert_quote_to_invoice))
        .route("/quotes/{id}/convert-to-delivery-note", post(convert_quote_to_delivery_note))
        .route("/quotes/{id}/duplicate", post(duplicate_quote))
}

async fn get_all_quotes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Quote>>, (StatusCode, String)> {
    let quotes = repository::get_all_quotes(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(quotes))
}

async fn get_quote_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Quote>, (StatusCode, String)> {
    let quote = repository::get_quote_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(quote))
}

async fn create_quote(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateQuoteInput>,
) -> Result<Json<Quote>, (StatusCode, String)> {
    let quote = repository::create_quote(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(quote))
}

async fn update_quote(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateQuoteInput>,
) -> Result<Json<Quote>, (StatusCode, String)> {
    input.id = id;
    let quote = repository::update_quote(&state.pool, input, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(quote))
}

async fn delete_quote(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_quote(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn batch_delete_quotes(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = repository::batch_delete_quotes(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(count))
}

async fn convert_quote_to_invoice(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::convert_quote_to_invoice(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}

async fn convert_quote_to_delivery_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeliveryNote>, (StatusCode, String)> {
    let delivery_note = repository::convert_quote_to_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(delivery_note))
}

async fn duplicate_quote(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Quote>, (StatusCode, String)> {
    let quote = repository::duplicate_quote(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(quote))
}
