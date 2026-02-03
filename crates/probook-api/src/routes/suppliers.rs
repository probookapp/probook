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
        .route("/suppliers", get(get_all_suppliers).post(create_supplier))
        .route("/suppliers/{id}", get(get_supplier_by_id).put(update_supplier).delete(delete_supplier))
        .route("/suppliers/batch-delete", post(batch_delete_suppliers))
}

async fn get_all_suppliers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Supplier>>, (StatusCode, String)> {
    let suppliers = repository::get_all_suppliers(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(suppliers))
}

async fn get_supplier_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Supplier>, (StatusCode, String)> {
    let supplier = repository::get_supplier_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(supplier))
}

async fn create_supplier(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateSupplierInput>,
) -> Result<Json<Supplier>, (StatusCode, String)> {
    let supplier = repository::create_supplier(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(supplier))
}

async fn update_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateSupplierInput>,
) -> Result<Json<Supplier>, (StatusCode, String)> {
    input.id = id;
    let supplier = repository::update_supplier(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(supplier))
}

async fn delete_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_supplier(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn batch_delete_suppliers(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = repository::batch_delete_suppliers(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(count))
}
