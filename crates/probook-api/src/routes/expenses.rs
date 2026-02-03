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
        .route("/expenses", get(get_all_expenses).post(create_expense))
        .route("/expenses/{id}", get(get_expense_by_id).put(update_expense).delete(delete_expense))
        .route("/expenses/batch-delete", post(batch_delete_expenses))
}

async fn get_all_expenses(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Expense>>, (StatusCode, String)> {
    let expenses = repository::get_all_expenses(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(expenses))
}

async fn get_expense_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Expense>, (StatusCode, String)> {
    let expense = repository::get_expense_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(expense))
}

async fn create_expense(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateExpenseInput>,
) -> Result<Json<Expense>, (StatusCode, String)> {
    let expense = repository::create_expense(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(expense))
}

async fn update_expense(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateExpenseInput>,
) -> Result<Json<Expense>, (StatusCode, String)> {
    input.id = id;
    let expense = repository::update_expense(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(expense))
}

async fn delete_expense(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_expense(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn batch_delete_expenses(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let count = repository::batch_delete_expenses(&state.pool, ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(count))
}
