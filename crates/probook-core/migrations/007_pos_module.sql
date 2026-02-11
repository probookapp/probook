-- POS Module Migration
-- Adds Point of Sale / Cash Register functionality

-- Add barcode field to products
ALTER TABLE products ADD COLUMN IF NOT EXISTS barcode TEXT;
CREATE INDEX IF NOT EXISTS idx_products_barcode ON products(barcode) WHERE barcode IS NOT NULL;

-- POS Registers (terminals)
CREATE TABLE IF NOT EXISTS pos_registers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT,
    machine_id TEXT UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- POS Sessions (daily cash sessions)
CREATE TABLE IF NOT EXISTS pos_sessions (
    id TEXT PRIMARY KEY,
    register_id TEXT NOT NULL REFERENCES pos_registers(id),
    user_id TEXT NOT NULL REFERENCES users(id),
    opened_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    closed_at TIMESTAMPTZ,
    opening_float DOUBLE PRECISION NOT NULL DEFAULT 0,
    expected_cash DOUBLE PRECISION,
    actual_cash DOUBLE PRECISION,
    cash_difference DOUBLE PRECISION,
    status TEXT NOT NULL DEFAULT 'OPEN',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- POS Transactions (sales tickets)
CREATE TABLE IF NOT EXISTS pos_transactions (
    id TEXT PRIMARY KEY,
    ticket_number TEXT NOT NULL UNIQUE,
    register_id TEXT NOT NULL REFERENCES pos_registers(id),
    session_id TEXT NOT NULL REFERENCES pos_sessions(id),
    client_id TEXT REFERENCES clients(id),
    user_id TEXT NOT NULL REFERENCES users(id),
    invoice_id TEXT REFERENCES invoices(id),
    transaction_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    subtotal_ht DOUBLE PRECISION NOT NULL DEFAULT 0,
    total_vat DOUBLE PRECISION NOT NULL DEFAULT 0,
    total_ttc DOUBLE PRECISION NOT NULL DEFAULT 0,
    discount_percent DOUBLE PRECISION DEFAULT 0,
    discount_amount DOUBLE PRECISION DEFAULT 0,
    final_amount DOUBLE PRECISION NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'COMPLETED',
    notes TEXT,
    synced BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- POS Transaction Lines
CREATE TABLE IF NOT EXISTS pos_transaction_lines (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL REFERENCES pos_transactions(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id),
    barcode TEXT,
    designation TEXT NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    unit_price_ht DOUBLE PRECISION NOT NULL,
    vat_rate DOUBLE PRECISION NOT NULL,
    total_ht DOUBLE PRECISION NOT NULL,
    total_vat DOUBLE PRECISION NOT NULL,
    total_ttc DOUBLE PRECISION NOT NULL,
    discount_percent DOUBLE PRECISION DEFAULT 0,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- POS Payments (multi-payment per transaction)
CREATE TABLE IF NOT EXISTS pos_payments (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL REFERENCES pos_transactions(id) ON DELETE CASCADE,
    payment_method TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    cash_given DOUBLE PRECISION,
    change_given DOUBLE PRECISION,
    card_reference TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Cash Movements (cash in/out)
CREATE TABLE IF NOT EXISTS pos_cash_movements (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL REFERENCES pos_sessions(id),
    user_id TEXT NOT NULL REFERENCES users(id),
    movement_type TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    reason TEXT NOT NULL,
    reference TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Printer Configurations
CREATE TABLE IF NOT EXISTS pos_printer_configs (
    id TEXT PRIMARY KEY,
    register_id TEXT REFERENCES pos_registers(id),
    printer_name TEXT NOT NULL,
    connection_type TEXT NOT NULL,
    connection_address TEXT NOT NULL,
    paper_width INTEGER NOT NULL DEFAULT 80,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- POS settings in company_settings
ALTER TABLE company_settings
    ADD COLUMN IF NOT EXISTS pos_ticket_prefix TEXT DEFAULT 'TK-',
    ADD COLUMN IF NOT EXISTS pos_auto_print_receipt BOOLEAN DEFAULT TRUE,
    ADD COLUMN IF NOT EXISTS pos_show_stock_warning BOOLEAN DEFAULT TRUE,
    ADD COLUMN IF NOT EXISTS pos_low_stock_threshold INTEGER DEFAULT 5;

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_pos_sessions_register ON pos_sessions(register_id);
CREATE INDEX IF NOT EXISTS idx_pos_sessions_status ON pos_sessions(status);
CREATE INDEX IF NOT EXISTS idx_pos_sessions_user ON pos_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_pos_transactions_session ON pos_transactions(session_id);
CREATE INDEX IF NOT EXISTS idx_pos_transactions_register ON pos_transactions(register_id);
CREATE INDEX IF NOT EXISTS idx_pos_transactions_date ON pos_transactions(transaction_date);
CREATE INDEX IF NOT EXISTS idx_pos_transactions_status ON pos_transactions(status);
CREATE INDEX IF NOT EXISTS idx_pos_transaction_lines_transaction ON pos_transaction_lines(transaction_id);
CREATE INDEX IF NOT EXISTS idx_pos_transaction_lines_product ON pos_transaction_lines(product_id);
CREATE INDEX IF NOT EXISTS idx_pos_payments_transaction ON pos_payments(transaction_id);
CREATE INDEX IF NOT EXISTS idx_pos_cash_movements_session ON pos_cash_movements(session_id);
