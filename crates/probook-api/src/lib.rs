pub mod middleware;
pub mod routes;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use axum::http::{header, Method};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

pub fn build_router(pool: PgPool, jwt_secret: &str, cors_origin: &str) -> Router {
    let state = Arc::new(AppState {
        pool,
        jwt_secret: jwt_secret.to_string(),
    });

    let cors = CorsLayer::new()
        .allow_origin(cors_origin.parse::<axum::http::HeaderValue>().unwrap_or_else(|_| {
            "http://localhost:1420".parse().unwrap()
        }))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::COOKIE,
        ])
        .allow_credentials(true);

    Router::new()
        .nest("/api", routes::api_router(state))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
