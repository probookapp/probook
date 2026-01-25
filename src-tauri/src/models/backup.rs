use serde::{Deserialize, Serialize};

use super::{Client, Product, Quote, Invoice, Payment, CompanySettings};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupData {
    pub version: String,
    pub created_at: String,
    pub clients: Vec<Client>,
    pub products: Vec<Product>,
    pub quotes: Vec<Quote>,
    pub invoices: Vec<Invoice>,
    pub payments: Vec<Payment>,
    pub settings: CompanySettings,
}
