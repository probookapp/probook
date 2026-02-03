use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use sqlx::PgPool;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use probook_core::db::repository;
use crate::db::{self, connection::{DbConfig, DbConfigSafe}};
use probook_core::models::*;
use probook_core::services::import::{self, ImportResult};

pub struct AppState {
    pub pool: PgPool,
    pub current_user_id: Mutex<Option<String>>,
}

fn require_auth(state: &AppState) -> Result<String, String> {
    let guard = state.current_user_id.lock().unwrap();
    guard.clone().ok_or_else(|| "Authentication required".to_string())
}

// Client Commands
#[tauri::command]
pub async fn get_clients(state: State<'_, AppState>) -> Result<Vec<Client>, String> {
    require_auth(&state)?;
    repository::get_all_clients(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_client(id: String, state: State<'_, AppState>) -> Result<Client, String> {
    require_auth(&state)?;
    repository::get_client_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_client(input: CreateClientInput, state: State<'_, AppState>) -> Result<Client, String> {
    require_auth(&state)?;
    repository::create_client(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_client(input: UpdateClientInput, state: State<'_, AppState>) -> Result<Client, String> {
    require_auth(&state)?;
    repository::update_client(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_client(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_client(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_clients(ids: Vec<String>, state: State<'_, AppState>) -> Result<u64, String> {
    require_auth(&state)?;
    repository::batch_delete_clients(&state.pool, ids)
        .await
        .map_err(|e| e.to_string())
}

// Product Commands
#[tauri::command]
pub async fn get_products(state: State<'_, AppState>) -> Result<Vec<Product>, String> {
    require_auth(&state)?;
    repository::get_all_products(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_product(id: String, state: State<'_, AppState>) -> Result<Product, String> {
    require_auth(&state)?;
    repository::get_product_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_product(input: CreateProductInput, state: State<'_, AppState>) -> Result<Product, String> {
    require_auth(&state)?;
    repository::create_product(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_product(input: UpdateProductInput, state: State<'_, AppState>) -> Result<Product, String> {
    require_auth(&state)?;
    repository::update_product(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_product(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_product(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_products(ids: Vec<String>, state: State<'_, AppState>) -> Result<u64, String> {
    require_auth(&state)?;
    repository::batch_delete_products(&state.pool, ids)
        .await
        .map_err(|e| e.to_string())
}

// Quote Commands
#[tauri::command]
pub async fn get_quotes(state: State<'_, AppState>) -> Result<Vec<Quote>, String> {
    require_auth(&state)?;
    repository::get_all_quotes(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_quote(id: String, state: State<'_, AppState>) -> Result<Quote, String> {
    require_auth(&state)?;
    repository::get_quote_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_quote(input: CreateQuoteInput, state: State<'_, AppState>) -> Result<Quote, String> {
    require_auth(&state)?;
    repository::create_quote(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_quote(input: UpdateQuoteInput, state: State<'_, AppState>) -> Result<Quote, String> {
    require_auth(&state)?;
    // Get the current logo to potentially snapshot it with the quote
    let logo_snapshot = get_logo_base64_internal(&state).await;

    repository::update_quote(&state.pool, input, logo_snapshot)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_quote(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_quote(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_quotes(ids: Vec<String>, state: State<'_, AppState>) -> Result<u64, String> {
    require_auth(&state)?;
    repository::batch_delete_quotes(&state.pool, ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn convert_quote_to_invoice(id: String, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::convert_quote_to_invoice(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn duplicate_quote(id: String, state: State<'_, AppState>) -> Result<Quote, String> {
    require_auth(&state)?;
    repository::duplicate_quote(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

// Invoice Commands
#[tauri::command]
pub async fn get_invoices(state: State<'_, AppState>) -> Result<Vec<Invoice>, String> {
    require_auth(&state)?;
    repository::get_all_invoices(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_invoice(id: String, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::get_invoice_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_invoice(input: CreateInvoiceInput, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::create_invoice(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_invoice(input: UpdateInvoiceInput, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::update_invoice(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_invoice(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_invoice(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_invoices(ids: Vec<String>, state: State<'_, AppState>) -> Result<u64, String> {
    require_auth(&state)?;
    repository::batch_delete_invoices(&state.pool, ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_invoice_paid(id: String, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::mark_invoice_paid(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn issue_invoice(id: String, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    // Get the current logo to snapshot it with the invoice
    let logo_snapshot = get_logo_base64_internal(&state).await;

    repository::issue_invoice(&state.pool, &id, logo_snapshot)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn duplicate_invoice(id: String, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::duplicate_invoice(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

// Internal helper to get logo base64 without the tauri::command wrapper
async fn get_logo_base64_internal(state: &State<'_, AppState>) -> Option<String> {
    let settings = repository::get_company_settings(&state.pool).await.ok()?;

    if let Some(logo_path) = settings.logo_path {
        if logo_path.is_empty() {
            return None;
        }
        let path = PathBuf::from(&logo_path);
        if path.exists() {
            let data = fs::read(&path).ok()?;
            let base64_data = BASE64.encode(&data);

            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("png")
                .to_lowercase();

            let mime_type = match extension.as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "webp" => "image/webp",
                _ => "image/png",
            };

            return Some(format!("data:{};base64,{}", mime_type, base64_data));
        }
    }
    None
}

#[tauri::command]
pub async fn verify_invoice_integrity(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    require_auth(&state)?;
    repository::verify_invoice_integrity(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

// Payment Commands
#[tauri::command]
pub async fn get_payments_by_invoice(invoice_id: String, state: State<'_, AppState>) -> Result<Vec<Payment>, String> {
    require_auth(&state)?;
    repository::get_payments_by_invoice(&state.pool, &invoice_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_payment(input: CreatePaymentInput, state: State<'_, AppState>) -> Result<Payment, String> {
    require_auth(&state)?;
    repository::create_payment(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_payment(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_payment(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

// Settings Commands
#[tauri::command]
pub async fn get_company_settings(state: State<'_, AppState>) -> Result<CompanySettings, String> {
    require_auth(&state)?;
    repository::get_company_settings(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_company_settings(input: UpdateCompanySettingsInput, state: State<'_, AppState>) -> Result<CompanySettings, String> {
    require_auth(&state)?;
    repository::update_company_settings(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn upload_logo(
    file_path: String,
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    require_auth(&state)?;
    // Get the app data directory
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    // Create logos directory if it doesn't exist
    let logos_dir = app_data_dir.join("logos");
    fs::create_dir_all(&logos_dir).map_err(|e| e.to_string())?;

    // Get the file extension from the source path
    let source_path = PathBuf::from(&file_path);
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    // Create a unique filename for the logo
    let logo_filename = format!("company_logo.{}", extension);
    let logo_path = logos_dir.join(&logo_filename);

    // Copy the file to the app data directory
    fs::copy(&source_path, &logo_path).map_err(|e| e.to_string())?;

    // Get the logo path as string
    let logo_path_str = logo_path.to_string_lossy().to_string();

    // Update the company settings with the logo path
    repository::update_logo_path(&state.pool, &logo_path_str)
        .await
        .map_err(|e| e.to_string())?;

    Ok(logo_path_str)
}

#[tauri::command]
pub async fn get_logo_base64(state: State<'_, AppState>) -> Result<Option<String>, String> {
    require_auth(&state)?;
    let settings = repository::get_company_settings(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(logo_path) = settings.logo_path {
        // Skip empty strings
        if logo_path.is_empty() {
            return Ok(None);
        }
        let path = PathBuf::from(&logo_path);
        if path.exists() {
            let data = fs::read(&path).map_err(|e| e.to_string())?;
            let base64_data = BASE64.encode(&data);

            // Determine the MIME type based on extension
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("png")
                .to_lowercase();

            let mime_type = match extension.as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "webp" => "image/webp",
                _ => "image/png",
            };

            return Ok(Some(format!("data:{};base64,{}", mime_type, base64_data)));
        }
    }

    Ok(None)
}

#[tauri::command]
pub async fn delete_logo(
    _app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    require_auth(&state)?;
    let settings = repository::get_company_settings(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Delete the logo file if it exists
    if let Some(logo_path) = settings.logo_path {
        let path = PathBuf::from(&logo_path);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
    }

    // Clear the logo path in the database
    repository::update_logo_path(&state.pool, "")
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Expense Commands
#[tauri::command]
pub async fn get_expenses(state: State<'_, AppState>) -> Result<Vec<Expense>, String> {
    require_auth(&state)?;
    repository::get_all_expenses(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_expense(id: String, state: State<'_, AppState>) -> Result<Expense, String> {
    require_auth(&state)?;
    repository::get_expense_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_expense(input: CreateExpenseInput, state: State<'_, AppState>) -> Result<Expense, String> {
    require_auth(&state)?;
    repository::create_expense(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_expense(input: UpdateExpenseInput, state: State<'_, AppState>) -> Result<Expense, String> {
    require_auth(&state)?;
    repository::update_expense(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_expense(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_expense(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_expenses(ids: Vec<String>, state: State<'_, AppState>) -> Result<u64, String> {
    require_auth(&state)?;
    repository::batch_delete_expenses(&state.pool, ids)
        .await
        .map_err(|e| e.to_string())
}

// Supplier Commands
#[tauri::command]
pub async fn get_suppliers(state: State<'_, AppState>) -> Result<Vec<Supplier>, String> {
    require_auth(&state)?;
    repository::get_all_suppliers(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_supplier(id: String, state: State<'_, AppState>) -> Result<Supplier, String> {
    require_auth(&state)?;
    repository::get_supplier_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_supplier(input: CreateSupplierInput, state: State<'_, AppState>) -> Result<Supplier, String> {
    require_auth(&state)?;
    repository::create_supplier(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_supplier(input: UpdateSupplierInput, state: State<'_, AppState>) -> Result<Supplier, String> {
    require_auth(&state)?;
    repository::update_supplier(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_supplier(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_supplier(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_suppliers(ids: Vec<String>, state: State<'_, AppState>) -> Result<u64, String> {
    require_auth(&state)?;
    repository::batch_delete_suppliers(&state.pool, ids)
        .await
        .map_err(|e| e.to_string())
}

// Product-Supplier Link Commands
#[tauri::command]
pub async fn get_all_product_supplier_summaries(state: State<'_, AppState>) -> Result<Vec<ProductSupplierSummary>, String> {
    require_auth(&state)?;
    repository::get_all_product_supplier_summaries(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_suppliers_for_product(product_id: String, state: State<'_, AppState>) -> Result<Vec<SupplierWithPrice>, String> {
    require_auth(&state)?;
    repository::get_suppliers_for_product(&state.pool, &product_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_products_for_supplier(supplier_id: String, state: State<'_, AppState>) -> Result<Vec<ProductWithPrice>, String> {
    require_auth(&state)?;
    repository::get_products_for_supplier(&state.pool, &supplier_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_product_supplier(input: CreateProductSupplierInput, state: State<'_, AppState>) -> Result<ProductSupplier, String> {
    require_auth(&state)?;
    repository::add_product_supplier(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_product_supplier(link_id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::remove_product_supplier(&state.pool, &link_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_product_supplier_price(link_id: String, purchase_price_ht: f64, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::update_product_supplier_price(&state.pool, &link_id, purchase_price_ht)
        .await
        .map_err(|e| e.to_string())
}

// Dashboard Commands
#[tauri::command]
pub async fn get_dashboard_stats(state: State<'_, AppState>) -> Result<DashboardStats, String> {
    require_auth(&state)?;
    repository::get_dashboard_stats(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

// Backup Commands

// Derive a 256-bit key from password using Argon2id
fn derive_key_from_password(password: &str, salt: &[u8; 16]) -> Result<[u8; 32], String> {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| format!("Key derivation failed: {}", e))?;
    Ok(key)
}

// Decrypt an encrypted backup file
fn decrypt_backup(data: &[u8], password: &str) -> Result<BackupData, String> {
    if password.is_empty() {
        return Err("Password required for encrypted backup".to_string());
    }

    // Format: salt (16) + nonce (12) + ciphertext
    if data.len() < 16 + 12 {
        return Err("Invalid backup file".to_string());
    }

    let salt: [u8; 16] = data[0..16].try_into().map_err(|_| "Invalid backup file")?;
    let nonce_bytes: &[u8] = &data[16..28];
    let ciphertext = &data[28..];
    let nonce = Nonce::from_slice(nonce_bytes);

    let key = derive_key_from_password(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Failed to decrypt backup. Wrong password or corrupted file.")?;

    let json = String::from_utf8(plaintext).map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_backup(file_path: String, password: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    // Gather all data
    let clients = repository::get_all_clients(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let products = repository::get_all_products(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let quotes = repository::get_all_quotes(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let invoices = repository::get_all_invoices(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let payments = repository::get_all_payments(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let settings = repository::get_company_settings(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let expenses = repository::get_all_expenses(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let suppliers = repository::get_all_suppliers(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    // Gather all product-supplier links with real created_at
    let product_suppliers = repository::get_all_product_suppliers(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Gather users and permissions for backup
    let users = repository::get_all_users_for_backup(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let user_permissions = repository::get_all_user_permissions_for_backup(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Gather delivery notes, contacts, reminders, categories
    let delivery_notes = repository::get_all_delivery_notes(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let client_contacts = repository::get_all_client_contacts(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let reminders = repository::get_all_reminders(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let product_categories = repository::get_all_product_categories(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

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
        users,
        user_permissions,
        delivery_notes,
        client_contacts,
        reminders,
        product_categories,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&backup).map_err(|e| e.to_string())?;

    // Generate random salt for key derivation
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    // Derive key from password
    let key = derive_key_from_password(&password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;

    // Generate a random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, json.as_bytes())
        .map_err(|e| e.to_string())?;

    // Combine: salt (16 bytes) + nonce (12 bytes) + ciphertext
    let mut output = Vec::new();
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);

    // Write to file
    fs::write(&file_path, &output).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn import_backup(file_path: String, password: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    let data = fs::read(&file_path).map_err(|e| e.to_string())?;

    // Try to parse as plain JSON first (local backups)
    let backup: BackupData = if let Ok(json_str) = String::from_utf8(data.clone()) {
        if let Ok(backup) = serde_json::from_str(&json_str) {
            backup
        } else {
            // Not valid JSON, try encrypted format
            decrypt_backup(&data, &password)?
        }
    } else {
        // Not UTF-8, must be encrypted
        decrypt_backup(&data, &password)?
    };

    // Clear existing data and restore from backup in a single transaction
    // Note: This is a destructive operation
    let mut tx = state.pool.begin().await.map_err(|e| e.to_string())?;

    repository::clear_all_data(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // Restore clients
    for client in backup.clients {
        repository::restore_client(&mut *tx, client)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore product categories (before products, parents before children)
    let categories = backup.product_categories;
    let parents_first: Vec<_> = categories.iter().filter(|c| c.parent_id.is_none()).cloned().collect();
    let children: Vec<_> = categories.iter().filter(|c| c.parent_id.is_some()).cloned().collect();
    for category in parents_first.into_iter().chain(children.into_iter()) {
        repository::restore_product_category(&mut *tx, category)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore products
    for product in backup.products {
        repository::restore_product(&mut *tx, product)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore suppliers
    for supplier in backup.suppliers {
        repository::restore_supplier(&mut *tx, supplier)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore product-supplier links
    for ps in backup.product_suppliers {
        repository::restore_product_supplier(&mut *tx, ps)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore quotes
    for quote in backup.quotes {
        repository::restore_quote(&mut *tx, quote)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore invoices
    for invoice in backup.invoices {
        repository::restore_invoice(&mut *tx, invoice)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore payments
    for payment in backup.payments {
        repository::restore_payment(&mut *tx, payment)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore delivery notes
    for dn in backup.delivery_notes {
        repository::restore_delivery_note(&mut *tx, dn)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore expenses
    for expense in backup.expenses {
        repository::restore_expense(&mut *tx, expense)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore users
    for user in backup.users {
        repository::restore_user(&mut *tx, user)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore user permissions
    for perm in backup.user_permissions {
        repository::restore_user_permission(&mut *tx, perm)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore client contacts
    for contact in backup.client_contacts {
        repository::restore_client_contact(&mut *tx, contact)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore reminders
    for reminder in backup.reminders {
        repository::restore_reminder(&mut *tx, reminder)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Restore settings
    repository::restore_settings(&mut *tx, backup.settings)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_app_settings(
    app_language: String,
    app_theme: String,
    auto_update_enabled: bool,
    state: State<'_, AppState>,
) -> Result<CompanySettings, String> {
    require_auth(&state)?;
    repository::update_app_settings(&state.pool, &app_language, &app_theme, auto_update_enabled)
        .await
        .map_err(|e| e.to_string())
}

// Local Backup Commands

/// Backup info returned to frontend
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackupInfo {
    pub filename: String,
    pub path: String,
    pub created_at: String,
    pub size_bytes: u64,
}

/// Get the backups directory path
fn get_backups_dir(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let backups_dir = app_data_dir.join("backups");
    fs::create_dir_all(&backups_dir).map_err(|e| e.to_string())?;

    Ok(backups_dir)
}

/// Create a local backup in the app's backups folder
#[tauri::command]
pub async fn create_local_backup(
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<BackupInfo, String> {
    require_auth(&state)?;
    let backups_dir = get_backups_dir(&app_handle)?;

    // Generate backup filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("probook-backup-{}.json", timestamp);
    let backup_path = backups_dir.join(&filename);

    // Gather all data
    let clients = repository::get_all_clients(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let products = repository::get_all_products(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let quotes = repository::get_all_quotes(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let invoices = repository::get_all_invoices(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let payments = repository::get_all_payments(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let settings = repository::get_company_settings(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let expenses = repository::get_all_expenses(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let suppliers = repository::get_all_suppliers(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    // Gather all product-supplier links with real created_at
    let product_suppliers_local = repository::get_all_product_suppliers(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Gather users and permissions for backup
    let mut users_backup = repository::get_all_users_for_backup(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let user_permissions_backup = repository::get_all_user_permissions_for_backup(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Gather delivery notes, contacts, reminders, categories
    let delivery_notes = repository::get_all_delivery_notes(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let client_contacts = repository::get_all_client_contacts(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let reminders = repository::get_all_reminders(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let product_categories = repository::get_all_product_categories(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Redact password hashes for local backups (not encrypted)
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
        product_suppliers: product_suppliers_local,
        users: users_backup,
        user_permissions: user_permissions_backup,
        delivery_notes,
        client_contacts,
        reminders,
        product_categories,
    };

    // Serialize to JSON and write to file (local backups are unencrypted)
    let json = serde_json::to_string_pretty(&backup).map_err(|e| e.to_string())?;
    fs::write(&backup_path, &json).map_err(|e| e.to_string())?;

    // Update last backup date in settings
    repository::update_last_backup_date(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Clean up old backups (keep last 10)
    cleanup_old_backups(&backups_dir, 10)?;

    let size_bytes = fs::metadata(&backup_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(BackupInfo {
        filename,
        path: backup_path.to_string_lossy().to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        size_bytes,
    })
}

/// Clean up old backups, keeping only the most recent N
fn cleanup_old_backups(backups_dir: &PathBuf, keep_count: usize) -> Result<(), String> {
    let mut backups: Vec<_> = fs::read_dir(backups_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .map(|ext| ext == "enc" || ext == "json")
                .unwrap_or(false)
        })
        .collect();

    // Sort by modification time (newest first)
    backups.sort_by(|a, b| {
        let time_a = a.metadata().and_then(|m| m.modified()).ok();
        let time_b = b.metadata().and_then(|m| m.modified()).ok();
        time_b.cmp(&time_a)
    });

    // Delete old backups beyond the keep count
    for backup in backups.iter().skip(keep_count) {
        let _ = fs::remove_file(backup.path());
    }

    Ok(())
}

/// Get list of available local backups
#[tauri::command]
pub async fn get_backup_list(app_handle: AppHandle, state: State<'_, AppState>) -> Result<Vec<BackupInfo>, String> {
    require_auth(&state)?;
    let backups_dir = get_backups_dir(&app_handle)?;

    let mut backups: Vec<BackupInfo> = fs::read_dir(&backups_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .map(|ext| ext == "enc" || ext == "json")
                .unwrap_or(false)
        })
        .filter_map(|entry| {
            let path = entry.path();
            let metadata = fs::metadata(&path).ok()?;
            let modified = metadata.modified().ok()?;
            let datetime: chrono::DateTime<chrono::Utc> = modified.into();

            Some(BackupInfo {
                filename: entry.file_name().to_string_lossy().to_string(),
                path: path.to_string_lossy().to_string(),
                created_at: datetime.to_rfc3339(),
                size_bytes: metadata.len(),
            })
        })
        .collect();

    // Sort by date (newest first)
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(backups)
}

/// Open the backups folder in the system file explorer
#[tauri::command]
pub async fn open_backups_folder(app_handle: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    let backups_dir = get_backups_dir(&app_handle)?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&backups_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&backups_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&backups_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Delete a specific backup file
#[tauri::command]
pub async fn delete_backup(path: String, app_handle: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    let backup_path = PathBuf::from(&path);

    // Safety check: only delete backup files with correct extension
    if backup_path.extension().map(|ext| ext != "enc" && ext != "json").unwrap_or(true) {
        return Err("Can only delete backup files".to_string());
    }

    // Safety check: verify the file is inside the backups directory
    let backups_dir = get_backups_dir(&app_handle)?;
    let canonical_backup = backup_path.canonicalize().map_err(|e| e.to_string())?;
    let canonical_dir = backups_dir.canonicalize().map_err(|e| e.to_string())?;
    if !canonical_backup.starts_with(&canonical_dir) {
        return Err("Can only delete files in the backups directory".to_string());
    }

    fs::remove_file(&backup_path).map_err(|e| e.to_string())?;

    Ok(())
}

// Product Category Commands
#[tauri::command]
pub async fn get_product_categories(state: State<'_, AppState>) -> Result<Vec<ProductCategory>, String> {
    require_auth(&state)?;
    repository::get_all_product_categories(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_product_category(id: String, state: State<'_, AppState>) -> Result<ProductCategory, String> {
    require_auth(&state)?;
    repository::get_product_category_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_product_category(input: CreateProductCategoryInput, state: State<'_, AppState>) -> Result<ProductCategory, String> {
    require_auth(&state)?;
    repository::create_product_category(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_product_category(input: UpdateProductCategoryInput, state: State<'_, AppState>) -> Result<ProductCategory, String> {
    require_auth(&state)?;
    repository::update_product_category(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_product_category(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_product_category(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

// Product Photo Commands
#[tauri::command]
pub async fn upload_product_photo(
    product_id: String,
    file_path: String,
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    require_auth(&state)?;
    // Get the app data directory
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    // Create product_photos directory if it doesn't exist
    let photos_dir = app_data_dir.join("product_photos");
    fs::create_dir_all(&photos_dir).map_err(|e| e.to_string())?;

    // Get the file extension from the source path
    let source_path = PathBuf::from(&file_path);
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    // Create a unique filename for the photo
    let photo_filename = format!("{}_{}.{}", product_id, chrono::Utc::now().timestamp(), extension);
    let photo_path = photos_dir.join(&photo_filename);

    // Copy the file to the app data directory
    fs::copy(&source_path, &photo_path).map_err(|e| e.to_string())?;

    // Get the photo path as string
    let photo_path_str = photo_path.to_string_lossy().to_string();

    // Update the product with the photo path
    repository::update_product_photo(&state.pool, &product_id, Some(&photo_path_str))
        .await
        .map_err(|e| e.to_string())?;

    Ok(photo_path_str)
}

#[tauri::command]
pub async fn get_product_photo_base64(product_id: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    require_auth(&state)?;
    let product = repository::get_product_by_id(&state.pool, &product_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(photo_path) = product.photo_path {
        if photo_path.is_empty() {
            return Ok(None);
        }
        let path = PathBuf::from(&photo_path);
        if path.exists() {
            let data = fs::read(&path).map_err(|e| e.to_string())?;
            let base64_data = BASE64.encode(&data);

            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("png")
                .to_lowercase();

            let mime_type = match extension.as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "webp" => "image/webp",
                _ => "image/png",
            };

            return Ok(Some(format!("data:{};base64,{}", mime_type, base64_data)));
        }
    }

    Ok(None)
}

#[tauri::command]
pub async fn delete_product_photo(
    product_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    require_auth(&state)?;
    let product = repository::get_product_by_id(&state.pool, &product_id)
        .await
        .map_err(|e| e.to_string())?;

    // Delete the photo file if it exists
    if let Some(photo_path) = product.photo_path {
        let path = PathBuf::from(&photo_path);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
    }

    // Clear the photo path in the database
    repository::update_product_photo(&state.pool, &product_id, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Delivery Note Commands
#[tauri::command]
pub async fn get_delivery_notes(state: State<'_, AppState>) -> Result<Vec<DeliveryNote>, String> {
    require_auth(&state)?;
    repository::get_all_delivery_notes(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_delivery_note(id: String, state: State<'_, AppState>) -> Result<DeliveryNote, String> {
    require_auth(&state)?;
    repository::get_delivery_note_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_delivery_note(input: CreateDeliveryNoteInput, state: State<'_, AppState>) -> Result<DeliveryNote, String> {
    require_auth(&state)?;
    repository::create_delivery_note(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_delivery_note(input: UpdateDeliveryNoteInput, state: State<'_, AppState>) -> Result<DeliveryNote, String> {
    require_auth(&state)?;
    repository::update_delivery_note(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_delivery_note(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_delete_delivery_notes(ids: Vec<String>, state: State<'_, AppState>) -> Result<u64, String> {
    require_auth(&state)?;
    repository::batch_delete_delivery_notes(&state.pool, ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn duplicate_delivery_note(id: String, state: State<'_, AppState>) -> Result<DeliveryNote, String> {
    require_auth(&state)?;
    repository::duplicate_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn convert_quote_to_delivery_note(id: String, state: State<'_, AppState>) -> Result<DeliveryNote, String> {
    require_auth(&state)?;
    repository::convert_quote_to_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn convert_invoice_to_delivery_note(id: String, state: State<'_, AppState>) -> Result<DeliveryNote, String> {
    require_auth(&state)?;
    repository::convert_invoice_to_delivery_note(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn convert_delivery_note_to_invoice(id: String, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::convert_delivery_note_to_invoice(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_invoice_from_delivery_notes(delivery_note_ids: Vec<String>, state: State<'_, AppState>) -> Result<Invoice, String> {
    require_auth(&state)?;
    repository::create_invoice_from_delivery_notes(&state.pool, delivery_note_ids)
        .await
        .map_err(|e| e.to_string())
}

// Client Contact Commands
#[tauri::command]
pub async fn get_client_contacts(state: State<'_, AppState>) -> Result<Vec<ClientContact>, String> {
    require_auth(&state)?;
    repository::get_all_client_contacts(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_client_contacts_by_client(client_id: String, state: State<'_, AppState>) -> Result<Vec<ClientContact>, String> {
    require_auth(&state)?;
    repository::get_client_contacts_by_client_id(&state.pool, &client_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_client_contact(id: String, state: State<'_, AppState>) -> Result<ClientContact, String> {
    require_auth(&state)?;
    repository::get_client_contact_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_client_contact(input: CreateClientContactInput, state: State<'_, AppState>) -> Result<ClientContact, String> {
    require_auth(&state)?;
    repository::create_client_contact(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_client_contact(input: UpdateClientContactInput, state: State<'_, AppState>) -> Result<ClientContact, String> {
    require_auth(&state)?;
    repository::update_client_contact(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_client_contact(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_client_contact(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_contacts(query: String, state: State<'_, AppState>) -> Result<Vec<ClientContact>, String> {
    require_auth(&state)?;
    repository::search_contacts(&state.pool, &query)
        .await
        .map_err(|e| e.to_string())
}

// Reminder Commands
#[tauri::command]
pub async fn get_reminders(state: State<'_, AppState>) -> Result<Vec<Reminder>, String> {
    require_auth(&state)?;
    repository::get_all_reminders(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_pending_reminders(state: State<'_, AppState>) -> Result<Vec<Reminder>, String> {
    require_auth(&state)?;
    repository::get_pending_reminders(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_reminders_by_document(document_type: String, document_id: String, state: State<'_, AppState>) -> Result<Vec<Reminder>, String> {
    require_auth(&state)?;
    repository::get_reminders_by_document(&state.pool, &document_type, &document_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_reminder(input: CreateReminderInput, state: State<'_, AppState>) -> Result<Reminder, String> {
    require_auth(&state)?;
    repository::create_reminder(&state.pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_reminder_sent(id: String, state: State<'_, AppState>) -> Result<Reminder, String> {
    require_auth(&state)?;
    repository::mark_reminder_sent(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_reminder(id: String, state: State<'_, AppState>) -> Result<(), String> {
    require_auth(&state)?;
    repository::delete_reminder(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_and_create_reminders(state: State<'_, AppState>) -> Result<Vec<Reminder>, String> {
    require_auth(&state)?;
    let mut all_reminders = Vec::new();

    // Create payment due reminders
    let payment_reminders = repository::create_payment_due_reminders(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    all_reminders.extend(payment_reminders);

    // Create quote expiring reminders
    let quote_reminders = repository::create_quote_expiring_reminders(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    all_reminders.extend(quote_reminders);

    Ok(all_reminders)
}

// Report Commands
#[tauri::command]
pub async fn get_revenue_by_month(
    start_date: Option<String>,
    end_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<RevenueByPeriod>, String> {
    require_auth(&state)?;
    let start = start_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let end = end_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    repository::get_revenue_by_month(&state.pool, start, end)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_revenue_by_client(
    start_date: Option<String>,
    end_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<RevenueByClient>, String> {
    require_auth(&state)?;
    let start = start_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let end = end_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    repository::get_revenue_by_client(&state.pool, start, end)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_product_sales(
    start_date: Option<String>,
    end_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ProductSales>, String> {
    require_auth(&state)?;
    let start = start_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let end = end_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    repository::get_product_sales(&state.pool, start, end)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_outstanding_payments(state: State<'_, AppState>) -> Result<Vec<OutstandingPayment>, String> {
    require_auth(&state)?;
    repository::get_outstanding_payments(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_quote_conversion_stats(
    start_date: Option<String>,
    end_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<QuoteConversionStats, String> {
    require_auth(&state)?;
    let start = start_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let end = end_date.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    repository::get_quote_conversion_stats(&state.pool, start, end)
        .await
        .map_err(|e| e.to_string())
}

// Alerts and Reminders
#[tauri::command]
pub async fn get_alerts_summary(state: State<'_, AppState>) -> Result<AlertsSummary, String> {
    require_auth(&state)?;
    repository::get_alerts_summary(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_quote_expired(quote_id: String, state: State<'_, AppState>) -> Result<Quote, String> {
    require_auth(&state)?;
    repository::mark_quote_expired(&state.pool, &quote_id)
        .await
        .map_err(|e| e.to_string())
}

// Import Commands
#[tauri::command]
pub async fn import_clients(file_path: String, state: State<'_, AppState>) -> Result<ImportResult, String> {
    require_auth(&state)?;
    let (headers, rows) = import::parse_file(&file_path)?;

    import::validate_columns(
        &headers,
        &["name"],
        &["email", "phone", "address", "city", "postal_code", "country", "siret", "vat_number", "notes"],
    )?;

    let mut result = ImportResult::new();

    // Fetch all clients once before the loop to avoid O(n*m)
    let mut existing_clients = repository::get_all_clients(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    for (i, row) in rows.iter().enumerate() {
        let name = match import::get_field(&headers, row, "name") {
            Some(n) => n,
            None => {
                result.errors.push(format!("Row {}: missing name", i + 2));
                continue;
            }
        };

        let email = import::get_field(&headers, row, "email");
        let phone = import::get_field(&headers, row, "phone");

        // Check for duplicate by name+phone or email
        let found = existing_clients.iter().find(|c| {
            (c.name == name && c.phone.as_deref() == phone.as_deref())
                || (email.is_some() && c.email.as_deref() == email.as_deref())
        });

        if let Some(existing_client) = found {
            // Update existing client
            let input = UpdateClientInput {
                id: existing_client.id.clone(),
                name: name.clone(),
                email: email.clone(),
                phone: phone.clone(),
                address: import::get_field(&headers, row, "address").or_else(|| existing_client.address.clone()),
                city: import::get_field(&headers, row, "city").or_else(|| existing_client.city.clone()),
                postal_code: import::get_field(&headers, row, "postal_code").or_else(|| existing_client.postal_code.clone()),
                country: import::get_field(&headers, row, "country").or_else(|| existing_client.country.clone()),
                siret: import::get_field(&headers, row, "siret").or_else(|| existing_client.siret.clone()),
                vat_number: import::get_field(&headers, row, "vat_number").or_else(|| existing_client.vat_number.clone()),
                notes: import::get_field(&headers, row, "notes").or_else(|| existing_client.notes.clone()),
            };
            match repository::update_client(&state.pool, input).await {
                Ok(_) => result.updated += 1,
                Err(e) => result.errors.push(format!("Row {}: {}", i + 2, e)),
            }
        } else {
            // Create new client
            let input = CreateClientInput {
                name,
                email,
                phone,
                address: import::get_field(&headers, row, "address"),
                city: import::get_field(&headers, row, "city"),
                postal_code: import::get_field(&headers, row, "postal_code"),
                country: import::get_field(&headers, row, "country"),
                siret: import::get_field(&headers, row, "siret"),
                vat_number: import::get_field(&headers, row, "vat_number"),
                notes: import::get_field(&headers, row, "notes"),
            };
            match repository::create_client(&state.pool, input).await {
                Ok(new_client) => {
                    existing_clients.push(new_client);
                    result.added += 1;
                }
                Err(e) => result.errors.push(format!("Row {}: {}", i + 2, e)),
            }
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn import_products(file_path: String, state: State<'_, AppState>) -> Result<ImportResult, String> {
    require_auth(&state)?;
    let (headers, rows) = import::parse_file(&file_path)?;

    import::validate_columns(
        &headers,
        &["designation", "unit_price_ht"],
        &["description", "vat_rate", "unit", "reference", "is_service", "quantity", "purchase_price_ht"],
    )?;

    let mut result = ImportResult::new();

    // Fetch all products once before the loop to avoid O(n*m)
    let existing_products = repository::get_all_products(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    for (i, row) in rows.iter().enumerate() {
        let designation = match import::get_field(&headers, row, "designation") {
            Some(n) => n,
            None => {
                result.errors.push(format!("Row {}: missing designation", i + 2));
                continue;
            }
        };

        let unit_price_ht = match import::get_field(&headers, row, "unit_price_ht") {
            Some(p) => match p.parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    result.errors.push(format!("Row {}: invalid unit_price_ht '{}'", i + 2, p));
                    continue;
                }
            },
            None => {
                result.errors.push(format!("Row {}: missing unit_price_ht", i + 2));
                continue;
            }
        };

        let reference = import::get_field(&headers, row, "reference");

        // Check for duplicate by reference (if provided) or designation
        let found = existing_products.iter().find(|p| {
            if let Some(ref ref_val) = reference {
                p.reference.as_deref() == Some(ref_val.as_str())
            } else {
                p.designation == designation
            }
        });

        if found.is_some() {
            // Products: insert-only, skip existing
            result.skipped += 1;
            continue;
        }

        let vat_rate = import::get_field(&headers, row, "vat_rate")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(20.0);

        let is_service = import::get_field(&headers, row, "is_service")
            .map(|v| v == "1" || v.to_lowercase() == "true" || v.to_lowercase() == "yes" || v.to_lowercase() == "oui")
            .unwrap_or(false);

        let quantity = import::get_field(&headers, row, "quantity")
            .and_then(|v| v.parse::<i32>().ok());

        let purchase_price_ht = import::get_field(&headers, row, "purchase_price_ht")
            .and_then(|v| v.parse::<f64>().ok());

        let input = CreateProductInput {
            designation,
            description: import::get_field(&headers, row, "description"),
            unit_price_ht,
            vat_rate,
            unit: import::get_field(&headers, row, "unit").unwrap_or_else(|| "unite".to_string()),
            reference,
            is_service,
            category_id: None,
            quantity,
            purchase_price_ht,
            description_html: None,
        };

        match repository::create_product(&state.pool, input).await {
            Ok(_) => result.added += 1,
            Err(e) => result.errors.push(format!("Row {}: {}", i + 2, e)),
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn import_suppliers(file_path: String, state: State<'_, AppState>) -> Result<ImportResult, String> {
    require_auth(&state)?;
    let (headers, rows) = import::parse_file(&file_path)?;

    import::validate_columns(
        &headers,
        &["name"],
        &["email", "phone", "address", "notes"],
    )?;

    let mut result = ImportResult::new();

    // Fetch all suppliers once before the loop to avoid O(n*m)
    let mut existing_suppliers = repository::get_all_suppliers(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    for (i, row) in rows.iter().enumerate() {
        let name = match import::get_field(&headers, row, "name") {
            Some(n) => n,
            None => {
                result.errors.push(format!("Row {}: missing name", i + 2));
                continue;
            }
        };

        let email = import::get_field(&headers, row, "email");

        // Check for duplicate by name+email
        let found = existing_suppliers.iter().find(|s| {
            s.name == name && s.email.as_deref() == email.as_deref()
        });

        if let Some(existing_supplier) = found {
            // Update existing supplier
            let input = UpdateSupplierInput {
                id: existing_supplier.id.clone(),
                name: name.clone(),
                email: email.clone(),
                phone: import::get_field(&headers, row, "phone").or_else(|| existing_supplier.phone.clone()),
                address: import::get_field(&headers, row, "address").or_else(|| existing_supplier.address.clone()),
                notes: import::get_field(&headers, row, "notes").or_else(|| existing_supplier.notes.clone()),
            };
            match repository::update_supplier(&state.pool, input).await {
                Ok(_) => result.updated += 1,
                Err(e) => result.errors.push(format!("Row {}: {}", i + 2, e)),
            }
        } else {
            // Create new supplier
            let input = CreateSupplierInput {
                name,
                email,
                phone: import::get_field(&headers, row, "phone"),
                address: import::get_field(&headers, row, "address"),
                notes: import::get_field(&headers, row, "notes"),
            };
            match repository::create_supplier(&state.pool, input).await {
                Ok(new_supplier) => {
                    existing_suppliers.push(new_supplier);
                    result.added += 1;
                }
                Err(e) => result.errors.push(format!("Row {}: {}", i + 2, e)),
            }
        }
    }

    Ok(result)
}

// Auth helper: hash a password with argon2
fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Failed to hash password: {}", e))?;
    Ok(hash.to_string())
}

// Auth helper: verify a password against a hash
fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = argon2::PasswordHash::new(hash)
        .map_err(|e| format!("Invalid password hash: {}", e))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

// Auth helper: require admin role from current session
async fn require_admin(state: &State<'_, AppState>) -> Result<User, String> {
    let user_id = {
        let lock = state.current_user_id.lock().map_err(|e| e.to_string())?;
        lock.clone().ok_or_else(|| "Not authenticated".to_string())?
    };
    let user = repository::get_user_by_id(&state.pool, &user_id)
        .await
        .map_err(|_| "User not found".to_string())?;
    if user.role != "admin" {
        return Err("Admin access required".to_string());
    }
    Ok(user)
}

// Auth Commands
#[tauri::command]
pub async fn check_setup_required(state: State<'_, AppState>) -> Result<bool, String> {
    let exists = repository::check_any_users_exist(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(!exists)
}

#[tauri::command]
pub async fn setup_admin(input: SetupInput, state: State<'_, AppState>) -> Result<UserInfo, String> {
    let exists = repository::check_any_users_exist(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    if exists {
        return Err("Setup already completed".to_string());
    }

    if input.password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = hash_password(&input.password)?;

    repository::create_user(&state.pool, &id, &input.username, &input.display_name, &password_hash, "admin")
        .await
        .map_err(|e| e.to_string())?;

    let all_perms: Vec<String> = ALL_PERMISSIONS.iter().map(|s| s.to_string()).collect();
    repository::set_user_permissions(&state.pool, &id, &all_perms)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut lock = state.current_user_id.lock().map_err(|e| e.to_string())?;
        *lock = Some(id.clone());
    }

    let user = repository::get_user_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())?;
    repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn login(input: LoginInput, state: State<'_, AppState>) -> Result<UserInfo, String> {
    let user = repository::get_user_by_username(&state.pool, &input.username)
        .await
        .map_err(|_| "Invalid username or password".to_string())?;

    if !user.is_active {
        return Err("Account is disabled".to_string());
    }

    let valid = verify_password(&input.password, &user.password_hash)?;
    if !valid {
        return Err("Invalid username or password".to_string());
    }

    {
        let mut lock = state.current_user_id.lock().map_err(|e| e.to_string())?;
        *lock = Some(user.id.clone());
    }

    repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<(), String> {
    let mut lock = state.current_user_id.lock().map_err(|e| e.to_string())?;
    *lock = None;
    Ok(())
}

#[tauri::command]
pub async fn get_current_user(state: State<'_, AppState>) -> Result<Option<UserInfo>, String> {
    let user_id = {
        let lock = state.current_user_id.lock().map_err(|e| e.to_string())?;
        lock.clone()
    };

    match user_id {
        Some(id) => {
            let user = repository::get_user_by_id(&state.pool, &id)
                .await
                .map_err(|e| e.to_string())?;
            let info = repository::build_user_info(&state.pool, &user)
                .await
                .map_err(|e| e.to_string())?;
            Ok(Some(info))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn get_users(state: State<'_, AppState>) -> Result<Vec<UserInfo>, String> {
    require_admin(&state).await?;
    let users = repository::get_all_users(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for user in &users {
        let info = repository::build_user_info(&state.pool, user)
            .await
            .map_err(|e| e.to_string())?;
        result.push(info);
    }
    Ok(result)
}

#[tauri::command]
pub async fn create_user_account(input: CreateUserInput, state: State<'_, AppState>) -> Result<UserInfo, String> {
    require_admin(&state).await?;

    if input.password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = hash_password(&input.password)?;

    repository::create_user(&state.pool, &id, &input.username, &input.display_name, &password_hash, &input.role)
        .await
        .map_err(|e| e.to_string())?;

    repository::set_user_permissions(&state.pool, &id, &input.permissions)
        .await
        .map_err(|e| e.to_string())?;

    let user = repository::get_user_by_id(&state.pool, &id)
        .await
        .map_err(|e| e.to_string())?;
    repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_user_account(input: UpdateUserInput, state: State<'_, AppState>) -> Result<UserInfo, String> {
    require_admin(&state).await?;

    repository::update_user(&state.pool, &input.id, &input.username, &input.display_name, &input.role, input.is_active)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(ref password) = input.password {
        if !password.is_empty() {
            if password.len() < 8 {
                return Err("Password must be at least 8 characters".to_string());
            }
            let password_hash = hash_password(password)?;
            repository::update_user_password(&state.pool, &input.id, &password_hash)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    repository::set_user_permissions(&state.pool, &input.id, &input.permissions)
        .await
        .map_err(|e| e.to_string())?;

    let user = repository::get_user_by_id(&state.pool, &input.id)
        .await
        .map_err(|e| e.to_string())?;
    repository::build_user_info(&state.pool, &user)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_user_account(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let admin = require_admin(&state).await?;

    if admin.id == id {
        return Err("Cannot delete your own account".to_string());
    }

    // Use a transaction with FOR UPDATE to prevent race condition on last admin deletion
    let mut tx = state.pool.begin().await.map_err(|e| e.to_string())?;

    let target_user: probook_core::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE id = $1 FOR UPDATE"
    )
    .bind(&id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if target_user.role == "admin" {
        let (admin_count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE role = 'admin' FOR UPDATE"
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
        if admin_count <= 1 {
            return Err("Cannot delete the last admin account".to_string());
        }
    }

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn change_own_password(current_password: String, new_password: String, state: State<'_, AppState>) -> Result<(), String> {
    let user_id = {
        let lock = state.current_user_id.lock().map_err(|e| e.to_string())?;
        lock.clone().ok_or_else(|| "Not authenticated".to_string())?
    };

    let user = repository::get_user_by_id(&state.pool, &user_id)
        .await
        .map_err(|e| e.to_string())?;

    let valid = verify_password(&current_password, &user.password_hash)?;
    if !valid {
        return Err("Current password is incorrect".to_string());
    }

    if new_password.len() < 8 {
        return Err("New password must be at least 8 characters".to_string());
    }

    let new_hash = hash_password(&new_password)?;
    repository::update_user_password(&state.pool, &user_id, &new_hash)
        .await
        .map_err(|e| e.to_string())
}

// Database setup commands
#[tauri::command]
pub async fn check_db_configured(app: AppHandle) -> Result<bool, String> {
    let config = db::connection::load_db_config(&app).map_err(|e| e.to_string())?;
    Ok(config.is_some())
}

#[tauri::command]
pub async fn test_db_connection(config: DbConfig) -> Result<(), String> {
    let pool = probook_core::db::connect_to_postgres(&config)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    pool.close().await;
    Ok(())
}

#[tauri::command]
pub async fn save_db_config(config: DbConfig, app: AppHandle) -> Result<String, String> {
    // Save config to file
    db::connection::save_db_config_to_file(&app, &config).map_err(|e| e.to_string())?;

    // Connect to DB and run migrations immediately so no restart is needed
    let pool = probook_core::db::connect_to_postgres(&config)
        .await
        .map_err(|e| e.to_string())?;
    probook_core::db::migrations::run_migrations(&pool)
        .await
        .map_err(|e| e.to_string())?;
    app.manage(AppState {
        pool,
        current_user_id: Mutex::new(None),
    });

    Ok("Database configuration saved.".to_string())
}

#[tauri::command]
pub async fn get_db_config(app: AppHandle) -> Result<Option<DbConfigSafe>, String> {
    let config = db::connection::load_db_config(&app).map_err(|e| e.to_string())?;
    Ok(config.map(|c| c.to_safe()))
}
