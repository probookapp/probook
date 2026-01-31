use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

/// Safe version of DbConfig for frontend (no password)
#[derive(Debug, Clone, Serialize)]
pub struct DbConfigSafe {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
}

impl DbConfig {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    pub fn to_safe(&self) -> DbConfigSafe {
        DbConfigSafe {
            host: self.host.clone(),
            port: self.port,
            database: self.database.clone(),
            username: self.username.clone(),
        }
    }
}

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
    let config: DbConfig = serde_json::from_str(&data)?;
    Ok(Some(config))
}

pub fn save_db_config_to_file(app: &AppHandle, config: &DbConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path(app)?;
    let data = serde_json::to_string_pretty(config)?;
    std::fs::write(&config_path, data)?;
    Ok(())
}

pub async fn connect_to_postgres(config: &DbConfig) -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_url())
        .await?;
    Ok(pool)
}

pub async fn init_database(app: &AppHandle) -> Result<Option<PgPool>, Box<dyn std::error::Error>> {
    let config = match load_db_config(app)? {
        Some(c) => c,
        None => return Ok(None),
    };

    let pool = connect_to_postgres(&config).await?;

    // Run migrations
    super::migrations::run_migrations(&pool).await?;

    Ok(Some(pool))
}
