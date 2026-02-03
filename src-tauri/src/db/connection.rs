use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use sqlx::PgPool;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub use probook_core::db::connection::{DbConfig, DbConfigSafe};

pub fn get_config_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("db_config.json"))
}

pub fn load_db_config(app: &AppHandle) -> Result<Option<DbConfig>, Box<dyn std::error::Error>> {
    let config_path = get_config_path(app)?;
    if !config_path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&config_path)?;
    let mut config: DbConfig = serde_json::from_str(&data)?;

    // Decode base64-encoded password if present
    if config.password.starts_with("b64:") {
        config.password = String::from_utf8(
            BASE64.decode(&config.password[4..]).unwrap_or_default()
        ).unwrap_or_default();
    }

    Ok(Some(config))
}

pub fn save_db_config_to_file(app: &AppHandle, config: &DbConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path(app)?;

    // Encode the password as base64 before saving to disk
    let mut config_to_save = config.clone();
    config_to_save.password = format!("b64:{}", BASE64.encode(&config.password));

    let data = serde_json::to_string_pretty(&config_to_save)?;
    std::fs::write(&config_path, data)?;
    Ok(())
}

pub async fn init_database(app: &AppHandle) -> Result<Option<PgPool>, Box<dyn std::error::Error>> {
    let config = match load_db_config(app)? {
        Some(c) => c,
        None => return Ok(None),
    };

    let pool = probook_core::db::connect_to_postgres(&config).await?;

    // Run migrations
    probook_core::db::migrations::run_migrations(&pool).await?;

    Ok(Some(pool))
}
