use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use super::client::Client;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliveryNoteStatus {
    DRAFT,
    DELIVERED,
    CANCELLED,
}

impl fmt::Display for DeliveryNoteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeliveryNoteStatus::DRAFT => write!(f, "DRAFT"),
            DeliveryNoteStatus::DELIVERED => write!(f, "DELIVERED"),
            DeliveryNoteStatus::CANCELLED => write!(f, "CANCELLED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryNote {
    pub id: String,
    pub delivery_note_number: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Client>,
    pub quote_id: Option<String>,
    pub invoice_id: Option<String>,
    pub status: DeliveryNoteStatus,
    pub issue_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub delivery_address: Option<String>,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub lines: Vec<DeliveryNoteLine>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeliveryNoteRow {
    pub id: String,
    pub delivery_note_number: String,
    pub client_id: String,
    pub quote_id: Option<String>,
    pub invoice_id: Option<String>,
    pub status: String,
    pub issue_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub delivery_address: Option<String>,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeliveryNoteLine {
    pub id: String,
    pub delivery_note_id: String,
    pub product_id: Option<String>,
    pub description: String,
    pub description_html: Option<String>,
    pub quantity: f64,
    pub unit: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryNoteInput {
    pub client_id: String,
    pub quote_id: Option<String>,
    pub invoice_id: Option<String>,
    pub issue_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub delivery_address: Option<String>,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub lines: Vec<CreateDeliveryNoteLineInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryNoteLineInput {
    pub product_id: Option<String>,
    pub description: String,
    pub description_html: Option<String>,
    pub quantity: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeliveryNoteInput {
    pub id: String,
    pub client_id: String,
    pub quote_id: Option<String>,
    pub invoice_id: Option<String>,
    pub status: DeliveryNoteStatus,
    pub issue_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub delivery_address: Option<String>,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub lines: Vec<CreateDeliveryNoteLineInput>,
}

impl DeliveryNoteLine {
    pub fn new(delivery_note_id: &str, input: CreateDeliveryNoteLineInput, position: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            delivery_note_id: delivery_note_id.to_string(),
            product_id: input.product_id,
            description: input.description,
            description_html: input.description_html,
            quantity: input.quantity,
            unit: input.unit,
            position,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_note_line_creation() {
        let input = CreateDeliveryNoteLineInput {
            product_id: Some("prod-1".to_string()),
            description: "Widget".to_string(),
            description_html: Some("<p>Widget</p>".to_string()),
            quantity: 10.0,
            unit: Some("pcs".to_string()),
        };
        let line = DeliveryNoteLine::new("dn-1", input, 2);
        assert_eq!(line.delivery_note_id, "dn-1");
        assert_eq!(line.description, "Widget");
        assert_eq!(line.description_html, Some("<p>Widget</p>".to_string()));
        assert!((line.quantity - 10.0).abs() < f64::EPSILON);
        assert_eq!(line.unit, Some("pcs".to_string()));
        assert_eq!(line.position, 2);
    }

    #[test]
    fn test_delivery_note_status_display() {
        assert_eq!(DeliveryNoteStatus::DRAFT.to_string(), "DRAFT");
        assert_eq!(DeliveryNoteStatus::DELIVERED.to_string(), "DELIVERED");
        assert_eq!(DeliveryNoteStatus::CANCELLED.to_string(), "CANCELLED");
    }
}
