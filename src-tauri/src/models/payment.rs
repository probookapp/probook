use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    pub id: String,
    pub invoice_id: String,
    pub amount: f64,
    pub payment_date: NaiveDate,
    pub payment_method: String,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentInput {
    pub invoice_id: String,
    pub amount: f64,
    pub payment_date: NaiveDate,
    pub payment_method: String,
    pub reference: Option<String>,
    pub notes: Option<String>,
}

impl Payment {
    pub fn new(input: CreatePaymentInput) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            invoice_id: input.invoice_id,
            amount: input.amount,
            payment_date: input.payment_date,
            payment_method: input.payment_method,
            reference: input.reference,
            notes: input.notes,
            created_at: Utc::now(),
        }
    }
}
