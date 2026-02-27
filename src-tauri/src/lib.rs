pub mod commands;
pub mod db;
pub mod services;

use commands::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                // Initialize licensing engine (works offline, no DB needed)
                if let Err(e) = probook_core::services::licensing::engine::initialize() {
                    eprintln!("License engine initialization failed: {}", e);
                }

                // Initialize offline queue for POS
                if let Some(app_data_dir) = handle.path().app_data_dir().ok() {
                    if let Err(e) = services::offline_queue::init_offline_queue(app_data_dir) {
                        eprintln!("Failed to initialize offline queue: {}", e);
                    }
                }

                match db::init_database(&handle).await {
                    Ok(Some(pool)) => {
                        // Start backup scheduler
                        let backups_dir = handle
                            .path()
                            .app_data_dir()
                            .map(|d| d.join("backups"))
                            .unwrap_or_default();
                        let _scheduler = services::backup_scheduler::BackupScheduler::start(
                            pool.clone(),
                            backups_dir,
                        );

                        handle.manage(AppState { pool, current_user_id: std::sync::Mutex::new(None) });
                    }
                    Ok(None) => {
                        // No DB config file — frontend will show Database Setup page
                    }
                    Err(e) => {
                        eprintln!("Database connection failed: {}. Frontend will show setup page.", e);
                    }
                }
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
            commands::batch_delete_clients,
            // Product commands
            commands::get_products,
            commands::get_product,
            commands::create_product,
            commands::update_product,
            commands::delete_product,
            commands::batch_delete_products,
            // Quote commands
            commands::get_quotes,
            commands::get_quote,
            commands::create_quote,
            commands::update_quote,
            commands::delete_quote,
            commands::batch_delete_quotes,
            commands::convert_quote_to_invoice,
            commands::duplicate_quote,
            // Invoice commands
            commands::get_invoices,
            commands::get_invoice,
            commands::create_invoice,
            commands::update_invoice,
            commands::delete_invoice,
            commands::batch_delete_invoices,
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
            commands::update_backup_settings,
            commands::upload_logo,
            commands::get_logo_base64,
            commands::delete_logo,
            // Expense commands
            commands::get_expenses,
            commands::get_expense,
            commands::create_expense,
            commands::update_expense,
            commands::delete_expense,
            commands::batch_delete_expenses,
            // Supplier commands
            commands::get_suppliers,
            commands::get_supplier,
            commands::create_supplier,
            commands::update_supplier,
            commands::delete_supplier,
            commands::batch_delete_suppliers,
            // Product-Supplier link commands
            commands::get_all_product_supplier_summaries,
            commands::get_suppliers_for_product,
            commands::get_products_for_supplier,
            commands::add_product_supplier,
            commands::remove_product_supplier,
            commands::update_product_supplier_price,
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
            commands::batch_delete_delivery_notes,
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
            // Import commands
            commands::import_clients,
            commands::import_products,
            commands::import_suppliers,
            // Auth commands
            commands::check_setup_required,
            commands::setup_admin,
            commands::login,
            commands::logout,
            commands::get_current_user,
            commands::get_users,
            commands::create_user_account,
            commands::update_user_account,
            commands::delete_user_account,
            commands::change_own_password,
            // Database setup commands
            commands::check_db_configured,
            commands::test_db_connection,
            commands::save_db_config,
            commands::get_db_config,
            // POS Register commands
            commands::get_pos_registers,
            commands::get_pos_register,
            commands::create_pos_register,
            commands::update_pos_register,
            commands::delete_pos_register,
            // POS Session commands
            commands::get_active_pos_session,
            commands::open_pos_session,
            commands::close_pos_session,
            commands::get_pos_session_summary,
            // POS Transaction commands
            commands::lookup_product_by_barcode,
            commands::create_pos_transaction,
            commands::get_pos_transaction,
            commands::cancel_pos_transaction,
            commands::get_pos_session_transactions,
            // POS Cash Movement commands
            commands::create_pos_cash_movement,
            commands::get_pos_session_cash_movements,
            // POS Printer Config commands
            commands::get_pos_printer_configs,
            commands::create_pos_printer_config,
            commands::update_pos_printer_config,
            commands::delete_pos_printer_config,
            // POS Report commands
            commands::get_daily_pos_report,
            // Thermal printer commands
            commands::list_printer_ports,
            commands::test_thermal_printer,
            commands::print_pos_receipt,
            // Offline queue commands
            commands::queue_offline_transaction,
            commands::get_pending_offline_count,
            commands::get_pending_offline_transactions,
            commands::mark_offline_transaction_synced,
            commands::mark_offline_transaction_failed,
            commands::delete_offline_transaction,
            commands::check_database_connection,
            // Licensing commands
            commands::check_license_status,
            commands::initialize_license,
            commands::start_trial,
            commands::import_license,
            commands::get_device_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
