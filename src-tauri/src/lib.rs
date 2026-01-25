pub mod commands;
pub mod db;
pub mod models;

use commands::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = db::init_database(&handle).await.expect("Failed to initialize database");
                handle.manage(AppState { pool });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Client commands
            commands::get_clients,
            commands::get_client,
            commands::create_client,
            commands::update_client,
            commands::delete_client,
            // Product commands
            commands::get_products,
            commands::get_product,
            commands::create_product,
            commands::update_product,
            commands::delete_product,
            // Quote commands
            commands::get_quotes,
            commands::get_quote,
            commands::create_quote,
            commands::update_quote,
            commands::delete_quote,
            commands::convert_quote_to_invoice,
            commands::duplicate_quote,
            // Invoice commands
            commands::get_invoices,
            commands::get_invoice,
            commands::create_invoice,
            commands::update_invoice,
            commands::delete_invoice,
            commands::mark_invoice_paid,
            commands::issue_invoice,
            commands::verify_invoice_integrity,
            commands::duplicate_invoice,
            // Payment commands
            commands::get_payments_by_invoice,
            commands::create_payment,
            commands::delete_payment,
            // Settings commands
            commands::get_company_settings,
            commands::update_company_settings,
            commands::update_app_settings,
            commands::upload_logo,
            commands::get_logo_base64,
            commands::delete_logo,
            // Dashboard commands
            commands::get_dashboard_stats,
            // Backup commands
            commands::export_backup,
            commands::import_backup,
            commands::create_local_backup,
            commands::get_backup_list,
            commands::open_backups_folder,
            commands::delete_backup,
            // Product Category commands
            commands::get_product_categories,
            commands::get_product_category,
            commands::create_product_category,
            commands::update_product_category,
            commands::delete_product_category,
            // Product Photo commands
            commands::upload_product_photo,
            commands::get_product_photo_base64,
            commands::delete_product_photo,
            // Delivery Note commands
            commands::get_delivery_notes,
            commands::get_delivery_note,
            commands::create_delivery_note,
            commands::update_delivery_note,
            commands::delete_delivery_note,
            commands::duplicate_delivery_note,
            commands::convert_quote_to_delivery_note,
            commands::convert_invoice_to_delivery_note,
            commands::convert_delivery_note_to_invoice,
            commands::create_invoice_from_delivery_notes,
            // Client Contact commands
            commands::get_client_contacts,
            commands::get_client_contacts_by_client,
            commands::get_client_contact,
            commands::create_client_contact,
            commands::update_client_contact,
            commands::delete_client_contact,
            commands::search_contacts,
            // Reminder commands
            commands::get_reminders,
            commands::get_pending_reminders,
            commands::get_reminders_by_document,
            commands::create_reminder,
            commands::mark_reminder_sent,
            commands::delete_reminder,
            commands::check_and_create_reminders,
            // Report commands
            commands::get_revenue_by_month,
            commands::get_revenue_by_client,
            commands::get_product_sales,
            commands::get_outstanding_payments,
            commands::get_quote_conversion_stats,
            // Alerts commands
            commands::get_alerts_summary,
            commands::mark_quote_expired,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
