use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductSupplier {
    pub id: String,
    pub product_id: String,
    pub supplier_id: String,
    pub purchase_price_ht: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductSupplierInput {
    pub product_id: String,
    pub supplier_id: String,
    pub purchase_price_ht: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupplierWithPrice {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub purchase_price_ht: f64,
    pub link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductWithPrice {
    pub id: String,
    pub designation: String,
    pub reference: Option<String>,
    pub unit_price_ht: f64,
    pub purchase_price_ht: f64,
    pub link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductSupplierSummary {
    pub product_id: String,
    pub supplier_id: String,
    pub supplier_name: String,
}

impl ProductSupplier {
    pub fn new(input: CreateProductSupplierInput) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            product_id: input.product_id,
            supplier_id: input.supplier_id,
            purchase_price_ht: input.purchase_price_ht,
            created_at: Utc::now(),
        }
    }
}
