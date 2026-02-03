-- Delivery notes and client contacts

CREATE TABLE IF NOT EXISTS delivery_notes (
    id TEXT PRIMARY KEY,
    delivery_note_number TEXT NOT NULL UNIQUE,
    client_id TEXT NOT NULL,
    quote_id TEXT,
    invoice_id TEXT,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    issue_date DATE NOT NULL,
    delivery_date DATE,
    delivery_address TEXT,
    notes TEXT,
    notes_html TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id),
    FOREIGN KEY (quote_id) REFERENCES quotes(id),
    FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);

CREATE TABLE IF NOT EXISTS delivery_note_lines (
    id TEXT PRIMARY KEY,
    delivery_note_id TEXT NOT NULL,
    product_id TEXT,
    description TEXT NOT NULL,
    description_html TEXT,
    quantity DOUBLE PRECISION NOT NULL,
    unit TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (delivery_note_id) REFERENCES delivery_notes(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE TABLE IF NOT EXISTS client_contacts (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    name TEXT NOT NULL,
    role TEXT,
    email TEXT,
    phone TEXT,
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE CASCADE
);
