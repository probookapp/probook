use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Client;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QuoteStatus {
    DRAFT,
    SENT,
    ACCEPTED,
    EXPIRED,
}

impl std::fmt::Display for QuoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteStatus::DRAFT => write!(f, "DRAFT"),
            QuoteStatus::SENT => write!(f, "SENT"),
            QuoteStatus::ACCEPTED => write!(f, "ACCEPTED"),
            QuoteStatus::EXPIRED => write!(f, "EXPIRED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub quote_number: String,
    pub client_id: String,
    pub client: Option<Client>,
    pub status: QuoteStatus,
    pub issue_date: NaiveDate,
    pub validity_date: NaiveDate,
    pub total_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub logo_snapshot: Option<String>,
    // Phase 2: Shipping costs
    pub shipping_cost_ht: f64,
    pub shipping_vat_rate: f64,
    // Phase 2: Down payment
    pub down_payment_percent: f64,
    pub down_payment_amount: f64,
    pub lines: Vec<QuoteLine>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuoteRow {
    pub id: String,
    pub quote_number: String,
    pub client_id: String,
    pub status: String,
    pub issue_date: NaiveDate,
    pub validity_date: NaiveDate,
    pub total_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub logo_snapshot: Option<String>,
    pub shipping_cost_ht: Option<f64>,
    pub shipping_vat_rate: Option<f64>,
    pub down_payment_percent: Option<f64>,
    pub down_payment_amount: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuoteLine {
    pub id: String,
    pub quote_id: String,
    pub product_id: Option<String>,
    pub description: String,
    pub quantity: f64,
    pub unit_price_ht: f64,
    pub vat_rate: f64,
    pub total_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub position: i32,
    // Phase 2: Subtotals
    pub group_name: Option<String>,
    pub is_subtotal_line: Option<bool>,
    // Phase 7: Rich text
    pub description_html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteLineInput {
    pub product_id: Option<String>,
    pub description: String,
    pub description_html: Option<String>,
    pub quantity: f64,
    pub unit_price_ht: f64,
    pub vat_rate: f64,
    pub group_name: Option<String>,
    pub is_subtotal_line: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteInput {
    pub client_id: String,
    pub issue_date: NaiveDate,
    pub validity_date: NaiveDate,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub shipping_cost_ht: Option<f64>,
    pub shipping_vat_rate: Option<f64>,
    pub down_payment_percent: Option<f64>,
    pub down_payment_amount: Option<f64>,
    pub lines: Vec<CreateQuoteLineInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuoteInput {
    pub id: String,
    pub client_id: String,
    pub status: QuoteStatus,
    pub issue_date: NaiveDate,
    pub validity_date: NaiveDate,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub shipping_cost_ht: Option<f64>,
    pub shipping_vat_rate: Option<f64>,
    pub down_payment_percent: Option<f64>,
    pub down_payment_amount: Option<f64>,
    pub lines: Vec<CreateQuoteLineInput>,
}

impl QuoteLine {
    pub fn new(quote_id: &str, input: CreateQuoteLineInput, position: i32) -> Self {
        let total_ht = input.quantity * input.unit_price_ht;
        let total_vat = total_ht * (input.vat_rate / 100.0);
        let total_ttc = total_ht + total_vat;

        Self {
            id: Uuid::new_v4().to_string(),
            quote_id: quote_id.to_string(),
            product_id: input.product_id,
            description: input.description,
            description_html: input.description_html,
            quantity: input.quantity,
            unit_price_ht: input.unit_price_ht,
            vat_rate: input.vat_rate,
            total_ht,
            total_vat,
            total_ttc,
            position,
            group_name: input.group_name,
            is_subtotal_line: input.is_subtotal_line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_line_input(qty: f64, price: f64, vat: f64) -> CreateQuoteLineInput {
        CreateQuoteLineInput {
            product_id: None,
            description: "Test".to_string(),
            description_html: None,
            quantity: qty,
            unit_price_ht: price,
            vat_rate: vat,
            group_name: None,
            is_subtotal_line: None,
        }
    }

    #[test]
    fn test_quote_line_total_calculations() {
        let line = QuoteLine::new("q-1", make_line_input(3.0, 200.0, 10.0), 0);
        assert!((line.total_ht - 600.0).abs() < f64::EPSILON);
        assert!((line.total_vat - 60.0).abs() < f64::EPSILON);
        assert!((line.total_ttc - 660.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_quote_line_zero_quantity() {
        let line = QuoteLine::new("q-1", make_line_input(0.0, 100.0, 20.0), 0);
        assert!((line.total_ht - 0.0).abs() < f64::EPSILON);
        assert!((line.total_vat - 0.0).abs() < f64::EPSILON);
        assert!((line.total_ttc - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_quote_status_display() {
        assert_eq!(QuoteStatus::DRAFT.to_string(), "DRAFT");
        assert_eq!(QuoteStatus::SENT.to_string(), "SENT");
        assert_eq!(QuoteStatus::ACCEPTED.to_string(), "ACCEPTED");
        assert_eq!(QuoteStatus::EXPIRED.to_string(), "EXPIRED");
    }
}
