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
        .route("/clients", get(get_all_clients).post(create_client))
        .route("/clients/{id}", get(get_client_by_id).put(update_client).delete(delete_client))
        .route("/clients/batch-delete", post(batch_delete_clients))
}

async fn get_all_clients(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Client>>, (StatusCode, String)> {
    let clients = repository::get_all_clients(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(clients))
}

async fn get_client_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let client = repository::get_client_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(client))
}

async fn create_client(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateClientInput>,
) -> Result<Json<Client>, (StatusCode, String)> {
    let client = repository::create_client(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(client))
}

async fn update_client(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateClientInput>,
) -> Result<Json<Client>, (StatusCode, String)> {
    input.id = id;
    let client = repository::update_client(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(client))
}

async fn delete_client(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_client(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn batch_delete_clients(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = repository::batch_delete_clients(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(count))
}
