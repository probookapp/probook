use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn default_auto_update() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CompanySettings {
    pub id: String,
    pub company_name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub siret: Option<String>,
    pub vat_number: Option<String>,
    pub logo_path: Option<String>,
    pub default_vat_rate: f64,
    pub default_payment_terms: i32,
    pub invoice_prefix: String,
    pub quote_prefix: String,
    pub next_invoice_number: i32,
    pub next_quote_number: i32,
    pub legal_mentions: Option<String>,
    pub legal_mentions_html: Option<String>,
    pub bank_details: Option<String>,
    // Phase 4: Delivery notes
    pub delivery_note_prefix: Option<String>,
    pub next_delivery_note_number: Option<i32>,
    // Phase 8: Cloud backup
    pub backup_schedule: Option<String>,
    pub last_backup_date: Option<String>,
    pub cloud_provider: Option<String>,
    #[serde(default)]
    pub auto_backup_enabled: bool,
    // Phase 9: Internationalization and theming
    pub app_language: Option<String>,
    pub app_theme: Option<String>,
    // Auto-update
    #[serde(default = "default_auto_update")]
    pub auto_update_enabled: bool,
    // Currency
    pub currency: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCompanySettingsInput {
    pub company_name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub siret: Option<String>,
    pub vat_number: Option<String>,
    pub default_vat_rate: f64,
    pub default_payment_terms: i32,
    pub invoice_prefix: String,
    pub quote_prefix: String,
    pub legal_mentions: Option<String>,
    pub legal_mentions_html: Option<String>,
    pub bank_details: Option<String>,
    pub currency: Option<String>,
    pub delivery_note_prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppSettingsInput {
    pub app_language: String,
    pub app_theme: String,
    pub auto_update_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_clients: i64,
    pub total_invoices: i64,
    pub total_quotes: i64,
    pub revenue_this_month: f64,
    pub revenue_this_year: f64,
    pub pending_payments: f64,
    pub total_expenses: f64,
    pub profit: f64,
    pub recent_invoices: Vec<super::Invoice>,
    pub recent_quotes: Vec<super::Quote>,
}
