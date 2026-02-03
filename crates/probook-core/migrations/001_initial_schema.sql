-- Core business tables: clients, products, categories, quotes, invoices, payments, company settings

CREATE TABLE IF NOT EXISTS clients (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    city TEXT,
    postal_code TEXT,
    country TEXT,
    siret TEXT,
    vat_number TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    designation TEXT NOT NULL,
    description TEXT,
    unit_price_ht DOUBLE PRECISION NOT NULL,
    vat_rate DOUBLE PRECISION NOT NULL DEFAULT 20.0,
    unit TEXT NOT NULL DEFAULT 'unité',
    reference TEXT,
    is_service BOOLEAN NOT NULL DEFAULT FALSE,
    category_id TEXT,
    photo_path TEXT,
    description_html TEXT,
    quantity INTEGER DEFAULT 0,
    purchase_price_ht DOUBLE PRECISION DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS product_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT REFERENCES product_categories(id),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS quotes (
    id TEXT PRIMARY KEY,
    quote_number TEXT NOT NULL UNIQUE,
    client_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    issue_date DATE NOT NULL,
    validity_date DATE NOT NULL,
    total_ht DOUBLE PRECISION NOT NULL DEFAULT 0,
    total_vat DOUBLE PRECISION NOT NULL DEFAULT 0,
    total_ttc DOUBLE PRECISION NOT NULL DEFAULT 0,
    notes TEXT,
    notes_html TEXT,
    logo_snapshot TEXT,
    shipping_cost_ht DOUBLE PRECISION DEFAULT 0,
    shipping_vat_rate DOUBLE PRECISION DEFAULT 20,
    down_payment_percent DOUBLE PRECISION DEFAULT 0,
    down_payment_amount DOUBLE PRECISION DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

CREATE TABLE IF NOT EXISTS quote_lines (
    id TEXT PRIMARY KEY,
    quote_id TEXT NOT NULL,
    product_id TEXT,
    description TEXT NOT NULL,
    description_html TEXT,
    quantity DOUBLE PRECISION NOT NULL,
    unit_price_ht DOUBLE PRECISION NOT NULL,
    vat_rate DOUBLE PRECISION NOT NULL,
    total_ht DOUBLE PRECISION NOT NULL,
    total_vat DOUBLE PRECISION NOT NULL,
    total_ttc DOUBLE PRECISION NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    group_name TEXT,
    is_subtotal_line BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (quote_id) REFERENCES quotes(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    invoice_number TEXT NOT NULL UNIQUE,
    client_id TEXT NOT NULL,
    quote_id TEXT,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    issue_date DATE NOT NULL,
    due_date DATE NOT NULL,
    total_ht DOUBLE PRECISION NOT NULL DEFAULT 0,
    total_vat DOUBLE PRECISION NOT NULL DEFAULT 0,
    total_ttc DOUBLE PRECISION NOT NULL DEFAULT 0,
    notes TEXT,
    notes_html TEXT,
    integrity_hash TEXT,
    logo_snapshot TEXT,
    shipping_cost_ht DOUBLE PRECISION DEFAULT 0,
    shipping_vat_rate DOUBLE PRECISION DEFAULT 20,
    down_payment_percent DOUBLE PRECISION DEFAULT 0,
    down_payment_amount DOUBLE PRECISION DEFAULT 0,
    is_down_payment_invoice BOOLEAN DEFAULT FALSE,
    parent_quote_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id),
    FOREIGN KEY (quote_id) REFERENCES quotes(id)
);

CREATE TABLE IF NOT EXISTS invoice_lines (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    product_id TEXT,
    description TEXT NOT NULL,
    description_html TEXT,
    quantity DOUBLE PRECISION NOT NULL,
    unit_price_ht DOUBLE PRECISION NOT NULL,
    vat_rate DOUBLE PRECISION NOT NULL,
    total_ht DOUBLE PRECISION NOT NULL,
    total_vat DOUBLE PRECISION NOT NULL,
    total_ttc DOUBLE PRECISION NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    group_name TEXT,
    is_subtotal_line BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    payment_date DATE NOT NULL,
    payment_method TEXT NOT NULL,
    reference TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS company_settings (
    id TEXT PRIMARY KEY DEFAULT 'default',
    company_name TEXT NOT NULL DEFAULT 'Mon Entreprise',
    address TEXT,
    city TEXT,
    postal_code TEXT,
    country TEXT DEFAULT 'France',
    phone TEXT,
    email TEXT,
    website TEXT,
    siret TEXT,
    vat_number TEXT,
    logo_path TEXT,
    default_vat_rate DOUBLE PRECISION NOT NULL DEFAULT 20.0,
    default_payment_terms INTEGER NOT NULL DEFAULT 30,
    invoice_prefix TEXT NOT NULL DEFAULT 'FA-',
    quote_prefix TEXT NOT NULL DEFAULT 'DE-',
    next_invoice_number INTEGER NOT NULL DEFAULT 1,
    next_quote_number INTEGER NOT NULL DEFAULT 1,
    legal_mentions TEXT,
    legal_mentions_html TEXT,
    bank_details TEXT,
    delivery_note_prefix TEXT DEFAULT 'BL-',
    next_delivery_note_number INTEGER DEFAULT 1,
    backup_schedule TEXT DEFAULT 'manual',
    last_backup_date TIMESTAMPTZ,
    cloud_provider TEXT,
    auto_backup_enabled BOOLEAN DEFAULT FALSE,
    app_language TEXT DEFAULT 'en',
    app_theme TEXT DEFAULT 'light',
    auto_update_enabled BOOLEAN DEFAULT TRUE,
    currency TEXT DEFAULT 'EUR',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO company_settings (id) VALUES ('default') ON CONFLICT DO NOTHING;
