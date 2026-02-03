use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Expense {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExpenseInput {
    pub name: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExpenseInput {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub notes: Option<String>,
}

impl Expense {
    pub fn new(input: CreateExpenseInput) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: input.name,
            amount: input.amount,
            date: input.date,
            notes: input.notes,
            created_at: now,
            updated_at: now,
        }
    }
}
