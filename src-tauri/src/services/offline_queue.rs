//! Offline queue service for storing POS transactions when database is unavailable
//! Uses local SQLite database for persistence

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OfflineQueueError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Queue is locked")]
    Locked,
    #[error("Transaction not found: {0}")]
    NotFound(String),
}

/// Status of a queued transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QueuedTransactionStatus {
    Pending,
    Syncing,
    Synced,
    Failed,
}

impl From<String> for QueuedTransactionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "PENDING" => QueuedTransactionStatus::Pending,
            "SYNCING" => QueuedTransactionStatus::Syncing,
            "SYNCED" => QueuedTransactionStatus::Synced,
            "FAILED" => QueuedTransactionStatus::Failed,
            _ => QueuedTransactionStatus::Pending,
        }
    }
}

impl From<QueuedTransactionStatus> for String {
    fn from(s: QueuedTransactionStatus) -> Self {
        match s {
            QueuedTransactionStatus::Pending => "PENDING".to_string(),
            QueuedTransactionStatus::Syncing => "SYNCING".to_string(),
            QueuedTransactionStatus::Synced => "SYNCED".to_string(),
            QueuedTransactionStatus::Failed => "FAILED".to_string(),
        }
    }
}

/// A transaction queued for sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedTransaction {
    pub id: String,
    pub transaction_data: String, // JSON-serialized CreatePosTransactionInput
    pub status: QueuedTransactionStatus,
    pub retry_count: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sync result for a batch of transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub synced_count: i32,
    pub failed_count: i32,
    pub errors: Vec<String>,
}

/// Offline queue manager
pub struct OfflineQueue {
    conn: Mutex<Connection>,
}

impl OfflineQueue {
    /// Create a new offline queue with the specified database path
    pub fn new(db_path: PathBuf) -> Result<Self, OfflineQueueError> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;

        // Initialize schema
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS offline_transactions (
                id TEXT PRIMARY KEY,
                transaction_data TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'PENDING',
                retry_count INTEGER NOT NULL DEFAULT 0,
                last_error TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_offline_transactions_status
                ON offline_transactions(status);

            CREATE TABLE IF NOT EXISTS offline_queue_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Queue a transaction for later sync
    pub fn queue_transaction(
        &self,
        id: &str,
        transaction_data: &str,
    ) -> Result<(), OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            r#"
            INSERT INTO offline_transactions (id, transaction_data, status, created_at, updated_at)
            VALUES (?1, ?2, 'PENDING', ?3, ?3)
            "#,
            params![id, transaction_data, now],
        )?;

        Ok(())
    }

    /// Get all pending transactions
    pub fn get_pending_transactions(&self) -> Result<Vec<QueuedTransaction>, OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;

        let mut stmt = conn.prepare(
            r#"
            SELECT id, transaction_data, status, retry_count, last_error, created_at, updated_at
            FROM offline_transactions
            WHERE status = 'PENDING' OR status = 'FAILED'
            ORDER BY created_at ASC
            "#,
        )?;

        let transactions = stmt
            .query_map([], |row| {
                Ok(QueuedTransaction {
                    id: row.get(0)?,
                    transaction_data: row.get(1)?,
                    status: row.get::<_, String>(2)?.into(),
                    retry_count: row.get(3)?,
                    last_error: row.get(4)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(transactions)
    }

    /// Get count of pending transactions
    pub fn get_pending_count(&self) -> Result<i64, OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM offline_transactions WHERE status = 'PENDING' OR status = 'FAILED'",
            [],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Mark a transaction as syncing
    pub fn mark_syncing(&self, id: &str) -> Result<(), OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE offline_transactions SET status = 'SYNCING', updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;

        Ok(())
    }

    /// Mark a transaction as synced (successfully)
    pub fn mark_synced(&self, id: &str) -> Result<(), OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE offline_transactions SET status = 'SYNCED', updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;

        Ok(())
    }

    /// Mark a transaction as failed
    pub fn mark_failed(&self, id: &str, error: &str) -> Result<(), OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;
        let now = Utc::now().to_rfc3339();

        conn.execute(
            r#"
            UPDATE offline_transactions
            SET status = 'FAILED',
                retry_count = retry_count + 1,
                last_error = ?1,
                updated_at = ?2
            WHERE id = ?3
            "#,
            params![error, now, id],
        )?;

        Ok(())
    }

    /// Delete a synced transaction from the queue
    pub fn delete_transaction(&self, id: &str) -> Result<(), OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;

        conn.execute("DELETE FROM offline_transactions WHERE id = ?1", params![id])?;

        Ok(())
    }

    /// Clean up old synced transactions (older than specified days)
    pub fn cleanup_old_synced(&self, days_old: i64) -> Result<i64, OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;
        let cutoff = (Utc::now() - chrono::Duration::days(days_old)).to_rfc3339();

        let deleted = conn.execute(
            "DELETE FROM offline_transactions WHERE status = 'SYNCED' AND updated_at < ?1",
            params![cutoff],
        )?;

        Ok(deleted as i64)
    }

    /// Get a specific transaction by ID
    pub fn get_transaction(&self, id: &str) -> Result<Option<QueuedTransaction>, OfflineQueueError> {
        let conn = self.conn.lock().map_err(|_| OfflineQueueError::Locked)?;

        let mut stmt = conn.prepare(
            r#"
            SELECT id, transaction_data, status, retry_count, last_error, created_at, updated_at
            FROM offline_transactions
            WHERE id = ?1
            "#,
        )?;

        let transaction = stmt
            .query_row(params![id], |row| {
                Ok(QueuedTransaction {
                    id: row.get(0)?,
                    transaction_data: row.get(1)?,
                    status: row.get::<_, String>(2)?.into(),
                    retry_count: row.get(3)?,
                    last_error: row.get(4)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })
            .optional()?;

        Ok(transaction)
    }
}

/// Global offline queue instance (initialized on app start)
static OFFLINE_QUEUE: std::sync::OnceLock<OfflineQueue> = std::sync::OnceLock::new();

/// Initialize the global offline queue
pub fn init_offline_queue(app_data_dir: PathBuf) -> Result<(), OfflineQueueError> {
    let db_path = app_data_dir.join("offline_queue.db");
    let queue = OfflineQueue::new(db_path)?;
    OFFLINE_QUEUE.set(queue).map_err(|_| {
        OfflineQueueError::Serialization("Failed to initialize offline queue".to_string())
    })?;
    Ok(())
}

/// Get the global offline queue instance
pub fn get_offline_queue() -> Option<&'static OfflineQueue> {
    OFFLINE_QUEUE.get()
}

/// Check if database connection is available
pub async fn check_db_connection(pool: &sqlx::PgPool) -> bool {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .is_ok()
}
