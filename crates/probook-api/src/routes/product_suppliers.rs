use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use std::sync::Arc;

use crate::AppState;
use probook_core::db::repository;
use probook_core::models::*;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/product-suppliers/summaries", get(get_all_product_supplier_summaries))
        .route("/product-suppliers/by-product/{productId}", get(get_suppliers_for_product))
        .route("/product-suppliers/by-supplier/{supplierId}", get(get_products_for_supplier))
        .route("/product-suppliers", post(add_product_supplier))
        .route("/product-suppliers/{id}", delete(remove_product_supplier))
        .route("/product-suppliers/{id}/price", put(update_product_supplier_price))
}

async fn get_all_product_supplier_summaries(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProductSupplierSummary>>, (StatusCode, String)> {
    let summaries = repository::get_all_product_supplier_summaries(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(summaries))
}

async fn get_suppliers_for_product(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<String>,
) -> Result<Json<Vec<SupplierWithPrice>>, (StatusCode, String)> {
    let suppliers = repository::get_suppliers_for_product(&state.pool, &product_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(suppliers))
}

async fn get_products_for_supplier(
    State(state): State<Arc<AppState>>,
    Path(supplier_id): Path<String>,
) -> Result<Json<Vec<ProductWithPrice>>, (StatusCode, String)> {
    let products = repository::get_products_for_supplier(&state.pool, &supplier_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(products))
}

async fn add_product_supplier(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateProductSupplierInput>,
) -> Result<Json<ProductSupplier>, (StatusCode, String)> {
    let link = repository::add_product_supplier(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(link))
}

async fn remove_product_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::remove_product_supplier(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn update_product_supplier_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<StatusCode, (StatusCode, String)> {
    let purchase_price_ht = body
        .get("purchase_price_ht")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing or invalid purchase_price_ht".to_string()))?;

    repository::update_product_supplier_price(&state.pool, &id, purchase_price_ht)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}
