use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use probook_core::db::repository;
use probook_core::models::*;

#[derive(Deserialize)]
pub struct UpdateAppSettingsInput {
    pub app_language: String,
    pub app_theme: String,
    pub auto_update_enabled: bool,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/settings", get(get_company_settings).put(update_company_settings))
        .route("/settings/app", put(update_app_settings))
}

async fn get_company_settings(
    State(state): State<Arc<AppState>>,
) -> Result<Json<CompanySettings>, (StatusCode, String)> {
    let settings = repository::get_company_settings(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(settings))
}

async fn update_company_settings(
    State(state): State<Arc<AppState>>,
    Json(input): Json<UpdateCompanySettingsInput>,
) -> Result<Json<CompanySettings>, (StatusCode, String)> {
    let settings = repository::update_company_settings(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(settings))
}

async fn update_app_settings(
    State(state): State<Arc<AppState>>,
    Json(input): Json<UpdateAppSettingsInput>,
) -> Result<Json<CompanySettings>, (StatusCode, String)> {
    let settings = repository::update_app_settings(
        &state.pool,
        &input.app_language,
        &input.app_theme,
        input.auto_update_enabled,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(settings))
}
