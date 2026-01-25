use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price_ht: f64,
    pub vat_rate: f64,
    pub unit: String,
    pub reference: Option<String>,
    pub is_service: bool,
    // Phase 3: Category and photo
    pub category_id: Option<String>,
    pub photo_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductInput {
    pub name: String,
    pub description: Option<String>,
    pub unit_price_ht: f64,
    pub vat_rate: f64,
    pub unit: String,
    pub reference: Option<String>,
    pub is_service: bool,
    pub category_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductInput {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price_ht: f64,
    pub vat_rate: f64,
    pub unit: String,
    pub reference: Option<String>,
    pub is_service: bool,
    pub category_id: Option<String>,
}

impl Product {
    pub fn new(input: CreateProductInput) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: input.name,
            description: input.description,
            unit_price_ht: input.unit_price_ht,
            vat_rate: input.vat_rate,
            unit: input.unit,
            reference: input.reference,
            is_service: input.is_service,
            category_id: input.category_id,
            photo_path: None,
            created_at: now,
            updated_at: now,
        }
    }
}
