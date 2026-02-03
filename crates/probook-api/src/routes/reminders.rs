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
        .route("/reminders", get(get_all_reminders).post(create_reminder))
        .route("/reminders/pending", get(get_pending_reminders))
        .route("/reminders/by-document/{documentType}/{documentId}", get(get_reminders_by_document))
        .route("/reminders/{id}/mark-sent", post(mark_reminder_sent))
        .route("/reminders/{id}", delete(delete_reminder))
        .route("/reminders/check-and-create", post(check_and_create_reminders))
}

async fn get_all_reminders(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Reminder>>, (StatusCode, String)> {
    let reminders = repository::get_all_reminders(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(reminders))
}

async fn get_pending_reminders(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Reminder>>, (StatusCode, String)> {
    let reminders = repository::get_pending_reminders(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(reminders))
}

async fn get_reminders_by_document(
    State(state): State<Arc<AppState>>,
    Path((document_type, document_id)): Path<(String, String)>,
) -> Result<Json<Vec<Reminder>>, (StatusCode, String)> {
    let reminders = repository::get_reminders_by_document(&state.pool, &document_type, &document_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(reminders))
}

async fn create_reminder(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateReminderInput>,
) -> Result<Json<Reminder>, (StatusCode, String)> {
    let reminder = repository::create_reminder(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(reminder))
}

async fn mark_reminder_sent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Reminder>, (StatusCode, String)> {
    let reminder = repository::mark_reminder_sent(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(reminder))
}

async fn delete_reminder(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_reminder(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn check_and_create_reminders(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Reminder>>, (StatusCode, String)> {
    let mut all_reminders = Vec::new();

    let payment_reminders = repository::create_payment_due_reminders(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    all_reminders.extend(payment_reminders);

    let quote_reminders = repository::create_quote_expiring_reminders(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    all_reminders.extend(quote_reminders);

    Ok(Json(all_reminders))
}
