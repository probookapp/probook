use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use probook_core::db::repository;
use probook_core::models::*;

#[derive(Deserialize)]
pub struct DateRangeQuery {
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/reports/revenue-by-month", get(get_revenue_by_month))
        .route("/reports/revenue-by-client", get(get_revenue_by_client))
        .route("/reports/product-sales", get(get_product_sales))
        .route("/reports/outstanding-payments", get(get_outstanding_payments))
        .route("/reports/quote-conversion", get(get_quote_conversion_stats))
}

fn parse_date(s: &Option<String>) -> Option<chrono::NaiveDate> {
    s.as_ref()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
}

async fn get_revenue_by_month(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<Vec<RevenueByPeriod>>, (StatusCode, String)> {
    let start_date = parse_date(&params.start_date);
    let end_date = parse_date(&params.end_date);
    let data = repository::get_revenue_by_month(&state.pool, start_date, end_date)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(data))
}

async fn get_revenue_by_client(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<Vec<RevenueByClient>>, (StatusCode, String)> {
    let start_date = parse_date(&params.start_date);
    let end_date = parse_date(&params.end_date);
    let data = repository::get_revenue_by_client(&state.pool, start_date, end_date)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(data))
}

async fn get_product_sales(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<Vec<ProductSales>>, (StatusCode, String)> {
    let start_date = parse_date(&params.start_date);
    let end_date = parse_date(&params.end_date);
    let data = repository::get_product_sales(&state.pool, start_date, end_date)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(data))
}

async fn get_outstanding_payments(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<OutstandingPayment>>, (StatusCode, String)> {
    let data = repository::get_outstanding_payments(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(data))
}

async fn get_quote_conversion_stats(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<QuoteConversionStats>, (StatusCode, String)> {
    let start_date = parse_date(&params.start_date);
    let end_date = parse_date(&params.end_date);
    let data = repository::get_quote_conversion_stats(&state.pool, start_date, end_date)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(data))
}
