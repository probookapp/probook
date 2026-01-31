use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const ALL_PERMISSIONS: &[&str] = &[
    "dashboard",
    "clients",
    "products",
    "suppliers",
    "quotes",
    "invoices",
    "delivery_notes",
    "phonebook",
    "reports",
    "expenses",
    "settings",
];

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub is_active: bool,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserPermission {
    pub id: String,
    pub user_id: String,
    pub permission_key: String,
    pub granted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupInput {
    pub username: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub role: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub password: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordInput {
    pub current_password: String,
    pub new_password: String,
}

/// User data for backup serialization (includes password_hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBackup {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Permission data for backup serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissionBackup {
    pub id: String,
    pub user_id: String,
    pub permission_key: String,
    pub granted: bool,
}
