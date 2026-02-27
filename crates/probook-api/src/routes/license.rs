use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use probook_core::services::licensing::engine::{self as licensing_engine, LicenseStatusInfo};
use std::sync::Arc;

use crate::AppState;

pub fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/license/status", get(get_license_status))
        .route("/license/initialize", post(initialize_license))
        .route("/license/start-trial", post(start_trial))
        .route("/license/import", post(import_license))
        .route("/license/device-id", get(get_device_id))
}

async fn get_license_status() -> Result<Json<LicenseStatusInfo>, (StatusCode, String)> {
    licensing_engine::get_status()
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn initialize_license() -> Result<Json<LicenseStatusInfo>, (StatusCode, String)> {
    licensing_engine::initialize()
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn start_trial() -> Result<Json<LicenseStatusInfo>, (StatusCode, String)> {
    licensing_engine::start_trial()
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))
}

async fn import_license(
    mut multipart: Multipart,
) -> Result<Json<LicenseStatusInfo>, (StatusCode, String)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read upload: {}", e)))?
    {
        if field.name() == Some("file") {
            let bytes = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read file: {}", e)))?;
            return licensing_engine::import_license(&bytes)
                .map(Json)
                .map_err(|e| (StatusCode::BAD_REQUEST, e));
        }
    }
    Err((
        StatusCode::BAD_REQUEST,
        "No license file provided".to_string(),
    ))
}

async fn get_device_id() -> Result<Json<String>, (StatusCode, String)> {
    licensing_engine::get_device_id()
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
