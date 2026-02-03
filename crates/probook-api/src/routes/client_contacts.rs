use axum::{
    extract::{Path, Query, State},
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
pub struct SearchQuery {
    pub query: Option<String>,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/contacts", get(get_all_client_contacts).post(create_client_contact))
        .route("/contacts/by-client/{clientId}", get(get_client_contacts_by_client))
        .route("/contacts/{id}", get(get_client_contact_by_id).put(update_client_contact).delete(delete_client_contact))
        .route("/contacts/search", get(search_contacts))
}

async fn get_all_client_contacts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ClientContact>>, (StatusCode, String)> {
    let contacts = repository::get_all_client_contacts(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(contacts))
}

async fn get_client_contacts_by_client(
    State(state): State<Arc<AppState>>,
    Path(client_id): Path<String>,
) -> Result<Json<Vec<ClientContact>>, (StatusCode, String)> {
    let contacts = repository::get_client_contacts_by_client_id(&state.pool, &client_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(contacts))
}

async fn get_client_contact_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ClientContact>, (StatusCode, String)> {
    let contact = repository::get_client_contact_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(contact))
}

async fn create_client_contact(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateClientContactInput>,
) -> Result<Json<ClientContact>, (StatusCode, String)> {
    let contact = repository::create_client_contact(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(contact))
}

async fn update_client_contact(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(mut input): Json<UpdateClientContactInput>,
) -> Result<Json<ClientContact>, (StatusCode, String)> {
    input.id = id;
    let contact = repository::update_client_contact(&state.pool, input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(contact))
}

async fn delete_client_contact(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_client_contact(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

async fn search_contacts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<ClientContact>>, (StatusCode, String)> {
    let query = params.query.unwrap_or_default();
    if query.is_empty() {
        let contacts = repository::get_all_client_contacts(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        return Ok(Json(contacts));
    }
    let contacts = repository::search_contacts(&state.pool, &query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(contacts))
}
