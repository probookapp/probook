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
        .route("/products", get(get_all_products).post(create_product))
        .route("/products/{id}", get(get_product_by_id).put(update_product).delete(delete_product))
        .route("/products/batch-delete", post(batch_delete_products))
        .route("/categories", get(get_all_product_categories).post(create_product_category))
        .route("/categories/{id}", get(get_product_category_by_id).put(update_product_category).delete(delete_product_category))
}

async fn get_all_products(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    let products = repository::get_all_products(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(products))
}

async fn get_product_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let product = repository::get_product_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(product))
}

async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateProductInput>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let product = repository::create_product(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(product))
}

async fn update_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateProductInput>,
) -> Result<Json<Product>, (StatusCode, String)> {
    input.id = id;
    let product = repository::update_product(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(product))
}

async fn delete_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_product(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn batch_delete_products(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = repository::batch_delete_products(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(count))
}

async fn get_all_product_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProductCategory>>, (StatusCode, String)> {
    let categories = repository::get_all_product_categories(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(categories))
}

async fn get_product_category_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ProductCategory>, (StatusCode, String)> {
    let category = repository::get_product_category_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(category))
}

async fn create_product_category(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateProductCategoryInput>,
) -> Result<Json<ProductCategory>, (StatusCode, String)> {
    let category = repository::create_product_category(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(category))
}

async fn update_product_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateProductCategoryInput>,
) -> Result<Json<ProductCategory>, (StatusCode, String)> {
    input.id = id;
    let category = repository::update_product_category(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(category))
}

async fn delete_product_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_product_category(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}
