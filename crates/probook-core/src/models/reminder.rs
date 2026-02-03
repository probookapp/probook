use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReminderType {
    PaymentDue,
    QuoteExpiring,
    DeliveryScheduled,
    Custom,
}

impl fmt::Display for ReminderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReminderType::PaymentDue => write!(f, "PAYMENT_DUE"),
            ReminderType::QuoteExpiring => write!(f, "QUOTE_EXPIRING"),
            ReminderType::DeliveryScheduled => write!(f, "DELIVERY_SCHEDULED"),
            ReminderType::Custom => write!(f, "CUSTOM"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentType {
    Invoice,
    Quote,
    DeliveryNote,
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentType::Invoice => write!(f, "INVOICE"),
            DocumentType::Quote => write!(f, "QUOTE"),
            DocumentType::DeliveryNote => write!(f, "DELIVERY_NOTE"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReminderRow {
    pub id: String,
    pub reminder_type: String,
    pub document_type: String,
    pub document_id: String,
    pub scheduled_date: NaiveDate,
    pub sent_at: Option<DateTime<Utc>>,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub reminder_type: ReminderType,
    pub document_type: DocumentType,
    pub document_id: String,
    pub scheduled_date: NaiveDate,
    pub sent_at: Option<DateTime<Utc>>,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReminderInput {
    pub reminder_type: ReminderType,
    pub document_type: DocumentType,
    pub document_id: String,
    pub scheduled_date: NaiveDate,
    pub message: Option<String>,
}

impl From<ReminderRow> for Reminder {
    fn from(row: ReminderRow) -> Self {
        let reminder_type = match row.reminder_type.as_str() {
            "PAYMENT_DUE" => ReminderType::PaymentDue,
            "QUOTE_EXPIRING" => ReminderType::QuoteExpiring,
            "DELIVERY_SCHEDULED" => ReminderType::DeliveryScheduled,
            _ => ReminderType::Custom,
        };

        let document_type = match row.document_type.as_str() {
            "INVOICE" => DocumentType::Invoice,
            "QUOTE" => DocumentType::Quote,
            "DELIVERY_NOTE" => DocumentType::DeliveryNote,
            _ => DocumentType::Invoice,
        };

        Reminder {
            id: row.id,
            reminder_type,
            document_type,
            document_id: row.document_id,
            scheduled_date: row.scheduled_date,
            sent_at: row.sent_at,
            message: row.message,
            created_at: row.created_at,
        }
    }
}

// Alert types for the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub alert_type: String, // "OVERDUE_INVOICE", "EXPIRING_QUOTE", "DUE_SOON"
    pub title: String,
    pub message: String,
    pub document_type: String,
    pub document_id: String,
    pub document_number: String,
    pub client_name: String,
    pub amount: Option<f64>,
    pub date: String, // due_date for invoices, validity_date for quotes
    pub days: i32,    // days overdue (positive) or days until due (negative)
    pub severity: String, // "warning", "danger", "info"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertsSummary {
    pub overdue_invoices: Vec<Alert>,
    pub due_soon_invoices: Vec<Alert>,
    pub expiring_quotes: Vec<Alert>,
    pub expired_quotes: Vec<Alert>,
    pub total_overdue_amount: f64,
    pub total_count: i32,
}
