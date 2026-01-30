use serde::{Deserialize, Serialize};

use super::{Client, Product, Quote, Invoice, Payment, CompanySettings, Expense, Supplier, ProductSupplier};

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
    #[serde(default)]
    pub expenses: Vec<Expense>,
    #[serde(default)]
    pub suppliers: Vec<Supplier>,
    #[serde(default)]
    pub product_suppliers: Vec<ProductSupplier>,
}
