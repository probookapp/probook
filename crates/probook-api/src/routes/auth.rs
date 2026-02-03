use axum::{
    extract::{Extension, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::Deserialize;
use std::sync::Arc;

use crate::middleware::auth::{create_jwt, AuthUser};
use crate::AppState;
use probook_core::db::repository;
use probook_core::models::*;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SetupRequest {
    pub username: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/setup", post(setup_admin))
        .route("/auth/setup-required", get(check_setup_required))
}

pub fn protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/me", get(get_current_user))
        .route("/auth/logout", post(logout))
        .route("/auth/change-password", post(change_password))
        .route("/auth/users", get(get_users).post(create_user))
        .route("/auth/users/{id}", axum::routing::put(update_user).delete(delete_user))
}

async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(input): Json<LoginRequest>,
) -> Result<(CookieJar, Json<UserInfo>), (StatusCode, String)> {
    let user = repository::get_user_by_username(&state.pool, &input.username)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()))?;

    if !user.is_active {
        return Err((StatusCode::FORBIDDEN, "Account is disabled".to_string()));
    }

    let valid = verify_password(&input.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()));
    }

    let token = create_jwt(&state, &user.id, &user.role)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let cookie = Cookie::build(("token", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::hours(24))
        .build();

    let user_info = repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((jar.add(cookie), Json(user_info)))
}

async fn setup_admin(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(input): Json<SetupRequest>,
) -> Result<(CookieJar, Json<UserInfo>), (StatusCode, String)> {
    let exists = repository::check_any_users_exist(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if exists {
        return Err((StatusCode::BAD_REQUEST, "Setup already completed".to_string()));
    }

    if input.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, "Password must be at least 8 characters".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = hash_password(&input.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    repository::create_user(&state.pool, &id, &input.username, &input.display_name, &password_hash, "admin")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let all_perms: Vec<String> = ALL_PERMISSIONS.iter().map(|s| s.to_string()).collect();
    repository::set_user_permissions(&state.pool, &id, &all_perms)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = create_jwt(&state, &id, "admin")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let cookie = Cookie::build(("token", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::hours(24))
        .build();

    let user = repository::get_user_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let user_info = repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((jar.add(cookie), Json(user_info)))
}

async fn check_setup_required(
    State(state): State<Arc<AppState>>,
) -> Result<Json<bool>, (StatusCode, String)> {
    let exists = repository::check_any_users_exist(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(!exists))
}

async fn get_current_user(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    let user = repository::get_user_by_id(&state.pool, &auth.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let info = repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(info))
}

async fn logout(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build("token").path("/").build())
}

async fn change_password(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(input): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user = repository::get_user_by_id(&state.pool, &auth.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let valid = verify_password(&input.current_password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    if !valid {
        return Err((StatusCode::BAD_REQUEST, "Current password is incorrect".to_string()));
    }

    if input.new_password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, "Password must be at least 8 characters".to_string()));
    }

    let new_hash = hash_password(&input.new_password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    repository::update_user_password(&state.pool, &auth.user_id, &new_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

async fn get_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<UserInfo>>, (StatusCode, String)> {
    let users = repository::get_all_users(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut result = Vec::new();
    for user in &users {
        let info = repository::build_user_info(&state.pool, user)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        result.push(info);
    }
    Ok(Json(result))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateUserInput>,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    if input.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, "Password must be at least 8 characters".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = hash_password(&input.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    repository::create_user(&state.pool, &id, &input.username, &input.display_name, &password_hash, &input.role)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !input.permissions.is_empty() {
        repository::set_user_permissions(&state.pool, &id, &input.permissions)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let user = repository::get_user_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let info = repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(info))
}

async fn update_user(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(input): Json<UpdateUserInput>,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    repository::update_user(&state.pool, &id, &input.username, &input.display_name, &input.role, input.is_active)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !input.permissions.is_empty() {
        repository::set_user_permissions(&state.pool, &id, &input.permissions)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let user = repository::get_user_by_id(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let info = repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(info))
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repository::delete_user(&state.pool, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

fn hash_password(password: &str) -> Result<String, String> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::SaltString;
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Failed to hash password: {}", e))?;
    Ok(hash.to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    use argon2::{Argon2, PasswordVerifier};
    let parsed_hash = argon2::PasswordHash::new(hash)
        .map_err(|e| format!("Invalid password hash: {}", e))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
