use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Client, Payment};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InvoiceStatus {
    DRAFT,
    ISSUED,
    PAID,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvoiceStatus::DRAFT => write!(f, "DRAFT"),
            InvoiceStatus::ISSUED => write!(f, "ISSUED"),
            InvoiceStatus::PAID => write!(f, "PAID"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub client_id: String,
    pub client: Option<Client>,
    pub quote_id: Option<String>,
    pub status: InvoiceStatus,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub integrity_hash: Option<String>,
    pub logo_snapshot: Option<String>,
    // Phase 2: Shipping costs
    pub shipping_cost_ht: f64,
    pub shipping_vat_rate: f64,
    // Phase 2: Down payment
    pub down_payment_percent: f64,
    pub down_payment_amount: f64,
    pub is_down_payment_invoice: bool,
    pub parent_quote_id: Option<String>,
    pub lines: Vec<InvoiceLine>,
    pub payments: Vec<Payment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InvoiceRow {
    pub id: String,
    pub invoice_number: String,
    pub client_id: String,
    pub quote_id: Option<String>,
    pub status: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub integrity_hash: Option<String>,
    pub logo_snapshot: Option<String>,
    pub shipping_cost_ht: Option<f64>,
    pub shipping_vat_rate: Option<f64>,
    pub down_payment_percent: Option<f64>,
    pub down_payment_amount: Option<f64>,
    pub is_down_payment_invoice: Option<bool>,
    pub parent_quote_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InvoiceLine {
    pub id: String,
    pub invoice_id: String,
    pub product_id: Option<String>,
    pub description: String,
    pub description_html: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceLineInput {
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
pub struct CreateInvoiceInput {
    pub client_id: String,
    pub quote_id: Option<String>,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub shipping_cost_ht: Option<f64>,
    pub shipping_vat_rate: Option<f64>,
    pub down_payment_percent: Option<f64>,
    pub down_payment_amount: Option<f64>,
    pub is_down_payment_invoice: Option<bool>,
    pub parent_quote_id: Option<String>,
    pub lines: Vec<CreateInvoiceLineInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoiceInput {
    pub id: String,
    pub client_id: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub notes: Option<String>,
    pub notes_html: Option<String>,
    pub shipping_cost_ht: Option<f64>,
    pub shipping_vat_rate: Option<f64>,
    pub down_payment_percent: Option<f64>,
    pub down_payment_amount: Option<f64>,
    pub lines: Vec<CreateInvoiceLineInput>,
}

impl InvoiceLine {
    pub fn new(invoice_id: &str, input: CreateInvoiceLineInput, position: i32) -> Self {
        let total_ht = input.quantity * input.unit_price_ht;
        let total_vat = total_ht * (input.vat_rate / 100.0);
        let total_ttc = total_ht + total_vat;

        Self {
            id: Uuid::new_v4().to_string(),
            invoice_id: invoice_id.to_string(),
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

    fn make_line_input(qty: f64, price: f64, vat: f64) -> CreateInvoiceLineInput {
        CreateInvoiceLineInput {
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
    fn test_invoice_line_total_calculations() {
        let line = InvoiceLine::new("inv-1", make_line_input(2.0, 100.0, 20.0), 0);
        assert!((line.total_ht - 200.0).abs() < f64::EPSILON);
        assert!((line.total_vat - 40.0).abs() < f64::EPSILON);
        assert!((line.total_ttc - 240.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_invoice_line_zero_vat() {
        let line = InvoiceLine::new("inv-1", make_line_input(5.0, 50.0, 0.0), 0);
        assert!((line.total_ht - 250.0).abs() < f64::EPSILON);
        assert!((line.total_vat - 0.0).abs() < f64::EPSILON);
        assert!((line.total_ttc - 250.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_invoice_line_fractional_quantity() {
        let line = InvoiceLine::new("inv-1", make_line_input(1.5, 100.0, 20.0), 0);
        assert!((line.total_ht - 150.0).abs() < f64::EPSILON);
        assert!((line.total_vat - 30.0).abs() < f64::EPSILON);
        assert!((line.total_ttc - 180.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_invoice_line_position_preserved() {
        let line = InvoiceLine::new("inv-1", make_line_input(1.0, 10.0, 20.0), 5);
        assert_eq!(line.position, 5);
        assert_eq!(line.invoice_id, "inv-1");
    }

    #[test]
    fn test_invoice_status_display() {
        assert_eq!(InvoiceStatus::DRAFT.to_string(), "DRAFT");
        assert_eq!(InvoiceStatus::ISSUED.to_string(), "ISSUED");
        assert_eq!(InvoiceStatus::PAID.to_string(), "PAID");
    }
}
