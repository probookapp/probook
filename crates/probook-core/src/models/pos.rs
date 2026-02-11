use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ========== POS Register ==========

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PosRegister {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub machine_id: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePosRegisterInput {
    pub name: String,
    pub location: Option<String>,
    pub machine_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePosRegisterInput {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub machine_id: Option<String>,
    pub is_active: bool,
}

impl PosRegister {
    pub fn new(input: CreatePosRegisterInput) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: input.name,
            location: input.location,
            machine_id: input.machine_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

// ========== POS Session ==========

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PosSession {
    pub id: String,
    pub register_id: String,
    pub user_id: String,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub opening_float: f64,
    pub expected_cash: Option<f64>,
    pub actual_cash: Option<f64>,
    pub cash_difference: Option<f64>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSessionInput {
    pub register_id: String,
    pub opening_float: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseSessionInput {
    pub session_id: String,
    pub actual_cash: f64,
    pub notes: Option<String>,
}

impl PosSession {
    pub fn new(register_id: String, user_id: String, opening_float: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            register_id,
            user_id,
            opened_at: now,
            closed_at: None,
            opening_float,
            expected_cash: None,
            actual_cash: None,
            cash_difference: None,
            status: "OPEN".to_string(),
            notes: None,
            created_at: now,
        }
    }
}

// ========== POS Transaction ==========

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PosTransactionRow {
    pub id: String,
    pub ticket_number: String,
    pub register_id: String,
    pub session_id: String,
    pub client_id: Option<String>,
    pub user_id: String,
    pub invoice_id: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub subtotal_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub discount_percent: f64,
    pub discount_amount: f64,
    pub final_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub synced: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosTransaction {
    pub id: String,
    pub ticket_number: String,
    pub register_id: String,
    pub session_id: String,
    pub client_id: Option<String>,
    pub user_id: String,
    pub invoice_id: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub subtotal_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub discount_percent: f64,
    pub discount_amount: f64,
    pub final_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub synced: bool,
    pub lines: Vec<PosTransactionLine>,
    pub payments: Vec<PosPayment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PosTransactionRow> for PosTransaction {
    fn from(row: PosTransactionRow) -> Self {
        Self {
            id: row.id,
            ticket_number: row.ticket_number,
            register_id: row.register_id,
            session_id: row.session_id,
            client_id: row.client_id,
            user_id: row.user_id,
            invoice_id: row.invoice_id,
            transaction_date: row.transaction_date,
            subtotal_ht: row.subtotal_ht,
            total_vat: row.total_vat,
            total_ttc: row.total_ttc,
            discount_percent: row.discount_percent,
            discount_amount: row.discount_amount,
            final_amount: row.final_amount,
            status: row.status,
            notes: row.notes,
            synced: row.synced,
            lines: Vec::new(),
            payments: Vec::new(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ========== POS Transaction Line ==========

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PosTransactionLine {
    pub id: String,
    pub transaction_id: String,
    pub product_id: Option<String>,
    pub barcode: Option<String>,
    pub designation: String,
    pub quantity: f64,
    pub unit_price_ht: f64,
    pub vat_rate: f64,
    pub total_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub discount_percent: f64,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionLineInput {
    pub product_id: Option<String>,
    pub barcode: Option<String>,
    pub designation: String,
    pub quantity: f64,
    pub unit_price_ht: f64,
    pub vat_rate: f64,
    pub discount_percent: Option<f64>,
}

impl PosTransactionLine {
    pub fn new(transaction_id: &str, input: CreateTransactionLineInput, position: i32) -> Self {
        let discount = input.discount_percent.unwrap_or(0.0);
        let base_ht = input.quantity * input.unit_price_ht;
        let total_ht = base_ht * (1.0 - discount / 100.0);
        let total_vat = total_ht * (input.vat_rate / 100.0);
        let total_ttc = total_ht + total_vat;

        Self {
            id: Uuid::new_v4().to_string(),
            transaction_id: transaction_id.to_string(),
            product_id: input.product_id,
            barcode: input.barcode,
            designation: input.designation,
            quantity: input.quantity,
            unit_price_ht: input.unit_price_ht,
            vat_rate: input.vat_rate,
            total_ht,
            total_vat,
            total_ttc,
            discount_percent: discount,
            position,
            created_at: Utc::now(),
        }
    }
}

// ========== POS Payment ==========

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PosPayment {
    pub id: String,
    pub transaction_id: String,
    pub payment_method: String,
    pub amount: f64,
    pub cash_given: Option<f64>,
    pub change_given: Option<f64>,
    pub card_reference: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePosPaymentInput {
    pub payment_method: String,
    pub amount: f64,
    pub cash_given: Option<f64>,
    pub card_reference: Option<String>,
}

impl PosPayment {
    pub fn new(transaction_id: &str, input: CreatePosPaymentInput) -> Self {
        let change = if input.payment_method == "CASH" {
            input.cash_given.map(|given| given - input.amount)
        } else {
            None
        };

        Self {
            id: Uuid::new_v4().to_string(),
            transaction_id: transaction_id.to_string(),
            payment_method: input.payment_method,
            amount: input.amount,
            cash_given: input.cash_given,
            change_given: change,
            card_reference: input.card_reference,
            created_at: Utc::now(),
        }
    }
}

// ========== Create Transaction Input ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePosTransactionInput {
    pub register_id: String,
    pub session_id: String,
    pub client_id: Option<String>,
    pub lines: Vec<CreateTransactionLineInput>,
    pub payments: Vec<CreatePosPaymentInput>,
    pub discount_percent: Option<f64>,
    pub discount_amount: Option<f64>,
    pub notes: Option<String>,
}

// ========== Cash Movement ==========

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PosCashMovement {
    pub id: String,
    pub session_id: String,
    pub user_id: String,
    pub movement_type: String,
    pub amount: f64,
    pub reason: String,
    pub reference: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCashMovementInput {
    pub session_id: String,
    pub movement_type: String,
    pub amount: f64,
    pub reason: String,
    pub reference: Option<String>,
}

impl PosCashMovement {
    pub fn new(input: CreateCashMovementInput, user_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id: input.session_id,
            user_id,
            movement_type: input.movement_type,
            amount: input.amount,
            reason: input.reason,
            reference: input.reference,
            created_at: Utc::now(),
        }
    }
}

// ========== Printer Configuration ==========

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PosPrinterConfig {
    pub id: String,
    pub register_id: Option<String>,
    pub printer_name: String,
    pub connection_type: String,
    pub connection_address: String,
    pub paper_width: i32,
    pub is_default: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrinterConfigInput {
    pub register_id: Option<String>,
    pub printer_name: String,
    pub connection_type: String,
    pub connection_address: String,
    pub paper_width: i32,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePrinterConfigInput {
    pub id: String,
    pub register_id: Option<String>,
    pub printer_name: String,
    pub connection_type: String,
    pub connection_address: String,
    pub paper_width: i32,
    pub is_default: bool,
    pub is_active: bool,
}

impl PosPrinterConfig {
    pub fn new(input: CreatePrinterConfigInput) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            register_id: input.register_id,
            printer_name: input.printer_name,
            connection_type: input.connection_type,
            connection_address: input.connection_address,
            paper_width: input.paper_width,
            is_default: input.is_default,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

// ========== Session Summary (Z-Report) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session: PosSession,
    pub register_name: String,
    pub user_name: String,
    pub transaction_count: i64,
    pub total_sales: f64,
    pub total_ht: f64,
    pub total_vat: f64,
    pub cash_sales: f64,
    pub card_sales: f64,
    pub cancelled_count: i64,
    pub cancelled_total: f64,
    pub cash_movements: Vec<PosCashMovement>,
    pub net_cash_movement: f64,
}

// ========== Daily Report ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPosReport {
    pub date: NaiveDate,
    pub register_id: Option<String>,
    pub register_name: Option<String>,
    pub session_count: i64,
    pub transaction_count: i64,
    pub total_sales: f64,
    pub total_ht: f64,
    pub total_vat: f64,
    pub cash_sales: f64,
    pub card_sales: f64,
    pub cancelled_count: i64,
    pub cancelled_total: f64,
}

// ========== Sync Result ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub synced: i64,
    pub failed: i64,
    pub remaining: i64,
}
