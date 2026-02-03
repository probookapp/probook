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
        .route("/delivery-notes", get(get_all_delivery_notes).post(create_delivery_note))
        .route("/delivery-notes/{id}", get(get_delivery_note_by_id).put(update_delivery_note).delete(delete_delivery_note))
        .route("/delivery-notes/batch-delete", post(batch_delete_delivery_notes))
        .route("/delivery-notes/{id}/duplicate", post(duplicate_delivery_note))
        .route("/delivery-notes/{id}/convert-to-invoice", post(convert_delivery_note_to_invoice))
}

async fn get_all_delivery_notes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DeliveryNote>>, (StatusCode, String)> {
    let delivery_notes = repository::get_all_delivery_notes(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(delivery_notes))
}

async fn get_delivery_note_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeliveryNote>, (StatusCode, String)> {
    let delivery_note = repository::get_delivery_note_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(delivery_note))
}

async fn create_delivery_note(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateDeliveryNoteInput>,
) -> Result<Json<DeliveryNote>, (StatusCode, String)> {
    let delivery_note = repository::create_delivery_note(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(delivery_note))
}

async fn update_delivery_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateDeliveryNoteInput>,
) -> Result<Json<DeliveryNote>, (StatusCode, String)> {
    input.id = id;
    let delivery_note = repository::update_delivery_note(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(delivery_note))
}

async fn delete_delivery_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn batch_delete_delivery_notes(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = repository::batch_delete_delivery_notes(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(count))
}

async fn duplicate_delivery_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeliveryNote>, (StatusCode, String)> {
    let delivery_note = repository::duplicate_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(delivery_note))
}

async fn convert_delivery_note_to_invoice(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, (StatusCode, String)> {
    let invoice = repository::convert_delivery_note_to_invoice(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(invoice))
}
