use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueByPeriod {
    pub period: String,
    pub revenue_ht: f64,
    pub revenue_ttc: f64,
    pub invoice_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueByClient {
    pub client_id: String,
    pub client_name: String,
    pub revenue_ht: f64,
    pub revenue_ttc: f64,
    pub invoice_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSales {
    pub product_id: String,
    pub product_name: String,
    pub quantity_sold: f64,
    pub revenue_ht: f64,
    pub revenue_ttc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutstandingPayment {
    pub invoice_id: String,
    pub invoice_number: String,
    pub client_name: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_ttc: f64,
    pub days_overdue: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteConversionStats {
    pub total_quotes: i64,
    pub converted_quotes: i64,
    pub conversion_rate: f64,
    pub total_quoted_amount: f64,
    pub converted_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilters {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub client_id: Option<String>,
}
