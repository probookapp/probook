use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

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

pub async fn connect_to_postgres(config: &DbConfig) -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_url())
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("non-UTF-8") {
                format!(
                    "Authentication failed. Check your username, password, and database name. \
                     (Host: {}:{}, Database: {})",
                    config.host, config.port, config.database
                )
            } else {
                msg
            }
        })?;
    Ok(pool)
}
