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
        .route("/alerts/summary", get(get_alerts_summary))
        .route("/alerts/mark-quote-expired/{quoteId}", post(mark_quote_expired))
}

async fn get_alerts_summary(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AlertsSummary>, (StatusCode, String)> {
    let summary = repository::get_alerts_summary(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(summary))
}

async fn mark_quote_expired(
    State(state): State<Arc<AppState>>,
    Path(quote_id): Path<String>,
) -> Result<Json<Quote>, (StatusCode, String)> {
    let quote = repository::mark_quote_expired(&state.pool, &quote_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(quote))
}
