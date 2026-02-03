use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

use probook_core::db::repository;
use probook_core::models::*;

pub struct BackupScheduler {
    shutdown: Arc<Notify>,
    handle: JoinHandle<()>,
}

impl BackupScheduler {
    pub fn start(pool: PgPool, backups_dir: PathBuf) -> Self {
        let shutdown = Arc::new(Notify::new());
        let shutdown_clone = shutdown.clone();

        let handle = tokio::spawn(async move {
            loop {
                // Read settings to check if auto backup is enabled
                let settings = match repository::get_company_settings(&pool).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("[backup-scheduler] Failed to read settings: {}", e);
                        // Retry in 5 minutes
                        tokio::select! {
                            _ = tokio::time::sleep(std::time::Duration::from_secs(300)) => continue,
                            _ = shutdown_clone.notified() => break,
                        }
                    }
                };

                if !settings.auto_backup_enabled {
                    // Auto backup disabled — check again in 10 minutes
                    tokio::select! {
                        _ = tokio::time::sleep(std::time::Duration::from_secs(600)) => continue,
                        _ = shutdown_clone.notified() => break,
                    }
                }

                // Calculate sleep duration based on schedule
                let interval_secs = match settings.backup_schedule.as_deref() {
                    Some("daily") => 24 * 60 * 60,
                    Some("weekly") => 7 * 24 * 60 * 60,
                    Some("monthly") => 30 * 24 * 60 * 60,
                    _ => {
                        // "manual" or unknown — don't auto-backup, check again later
                        tokio::select! {
                            _ = tokio::time::sleep(std::time::Duration::from_secs(600)) => continue,
                            _ = shutdown_clone.notified() => break,
                        }
                    }
                };

                // Check if a backup is actually due
                let should_backup = match settings.last_backup_date {
                    Some(last) => {
                        let elapsed = chrono::Utc::now() - last;
                        elapsed.num_seconds() >= interval_secs
                    }
                    None => true, // Never backed up
                };

                if should_backup {
                    match run_backup(&pool, &backups_dir).await {
                        Ok(()) => {
                            eprintln!("[backup-scheduler] Automatic backup completed successfully");
                        }
                        Err(e) => {
                            eprintln!("[backup-scheduler] Backup failed: {}", e);
                        }
                    }
                }

                // Sleep for 1 hour before checking again
                tokio::select! {
                    _ = tokio::time::sleep(std::time::Duration::from_secs(3600)) => {},
                    _ = shutdown_clone.notified() => break,
                }
            }
        });

        BackupScheduler { shutdown, handle }
    }

    pub fn stop(self) {
        self.shutdown.notify_one();
        drop(self.handle);
    }
}

async fn run_backup(pool: &PgPool, backups_dir: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(backups_dir).map_err(|e| e.to_string())?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("probook-backup-{}.json", timestamp);
    let backup_path = backups_dir.join(&filename);

    // Gather all data
    let clients = repository::get_all_clients(pool).await.map_err(|e| e.to_string())?;
    let products = repository::get_all_products(pool).await.map_err(|e| e.to_string())?;
    let quotes = repository::get_all_quotes(pool).await.map_err(|e| e.to_string())?;
    let invoices = repository::get_all_invoices(pool).await.map_err(|e| e.to_string())?;
    let payments = repository::get_all_payments(pool).await.map_err(|e| e.to_string())?;
    let settings = repository::get_company_settings(pool).await.map_err(|e| e.to_string())?;
    let expenses = repository::get_all_expenses(pool).await.map_err(|e| e.to_string())?;
    let suppliers = repository::get_all_suppliers(pool).await.map_err(|e| e.to_string())?;
    let product_suppliers = repository::get_all_product_suppliers(pool).await.map_err(|e| e.to_string())?;
    let mut users_backup = repository::get_all_users_for_backup(pool).await.map_err(|e| e.to_string())?;
    let user_permissions = repository::get_all_user_permissions_for_backup(pool).await.map_err(|e| e.to_string())?;
    let delivery_notes = repository::get_all_delivery_notes(pool).await.map_err(|e| e.to_string())?;
    let client_contacts = repository::get_all_client_contacts(pool).await.map_err(|e| e.to_string())?;
    let reminders = repository::get_all_reminders(pool).await.map_err(|e| e.to_string())?;
    let product_categories = repository::get_all_product_categories(pool).await.map_err(|e| e.to_string())?;

    // Redact password hashes
    for user in &mut users_backup {
        user.password_hash = "[redacted]".to_string();
    }

    let backup = BackupData {
        version: "2.0".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        clients,
        products,
        quotes,
        invoices,
        payments,
        settings,
        expenses,
        suppliers,
        product_suppliers,
        users: users_backup,
        user_permissions: user_permissions,
        delivery_notes,
        client_contacts,
        reminders,
        product_categories,
    };

    let json = serde_json::to_string_pretty(&backup).map_err(|e| e.to_string())?;
    std::fs::write(&backup_path, &json).map_err(|e| e.to_string())?;

    // Update last backup date
    repository::update_last_backup_date(pool).await.map_err(|e| e.to_string())?;

    // Clean up old backups (keep last 10)
    cleanup_old_backups(backups_dir, 10)?;

    Ok(())
}

fn cleanup_old_backups(backups_dir: &PathBuf, keep_count: usize) -> Result<(), String> {
    let mut backups: Vec<_> = std::fs::read_dir(backups_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .map(|ext| ext == "json")
                .unwrap_or(false)
        })
        .collect();

    if backups.len() <= keep_count {
        return Ok(());
    }

    backups.sort_by_key(|entry| {
        entry.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    let to_delete = backups.len() - keep_count;
    for entry in backups.iter().take(to_delete) {
        let _ = std::fs::remove_file(entry.path());
    }

    Ok(())
}
