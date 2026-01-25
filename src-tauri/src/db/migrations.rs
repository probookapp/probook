use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create clients table
    sqlx::query(
        r#"
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
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create products table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            unit_price_ht REAL NOT NULL,
            vat_rate REAL NOT NULL DEFAULT 20.0,
            unit TEXT NOT NULL DEFAULT 'unité',
            reference TEXT,
            is_service INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create quotes table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quotes (
            id TEXT PRIMARY KEY,
            quote_number TEXT NOT NULL UNIQUE,
            client_id TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'DRAFT',
            issue_date TEXT NOT NULL,
            validity_date TEXT NOT NULL,
            total_ht REAL NOT NULL DEFAULT 0,
            total_vat REAL NOT NULL DEFAULT 0,
            total_ttc REAL NOT NULL DEFAULT 0,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (client_id) REFERENCES clients(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create quote_lines table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quote_lines (
            id TEXT PRIMARY KEY,
            quote_id TEXT NOT NULL,
            product_id TEXT,
            description TEXT NOT NULL,
            quantity REAL NOT NULL,
            unit_price_ht REAL NOT NULL,
            vat_rate REAL NOT NULL,
            total_ht REAL NOT NULL,
            total_vat REAL NOT NULL,
            total_ttc REAL NOT NULL,
            position INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (quote_id) REFERENCES quotes(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create invoices table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS invoices (
            id TEXT PRIMARY KEY,
            invoice_number TEXT NOT NULL UNIQUE,
            client_id TEXT NOT NULL,
            quote_id TEXT,
            status TEXT NOT NULL DEFAULT 'DRAFT',
            issue_date TEXT NOT NULL,
            due_date TEXT NOT NULL,
            total_ht REAL NOT NULL DEFAULT 0,
            total_vat REAL NOT NULL DEFAULT 0,
            total_ttc REAL NOT NULL DEFAULT 0,
            notes TEXT,
            integrity_hash TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (client_id) REFERENCES clients(id),
            FOREIGN KEY (quote_id) REFERENCES quotes(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create invoice_lines table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS invoice_lines (
            id TEXT PRIMARY KEY,
            invoice_id TEXT NOT NULL,
            product_id TEXT,
            description TEXT NOT NULL,
            quantity REAL NOT NULL,
            unit_price_ht REAL NOT NULL,
            vat_rate REAL NOT NULL,
            total_ht REAL NOT NULL,
            total_vat REAL NOT NULL,
            total_ttc REAL NOT NULL,
            position INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create payments table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS payments (
            id TEXT PRIMARY KEY,
            invoice_id TEXT NOT NULL,
            amount REAL NOT NULL,
            payment_date TEXT NOT NULL,
            payment_method TEXT NOT NULL,
            reference TEXT,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create company_settings table
    sqlx::query(
        r#"
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
            default_vat_rate REAL NOT NULL DEFAULT 20.0,
            default_payment_terms INTEGER NOT NULL DEFAULT 30,
            invoice_prefix TEXT NOT NULL DEFAULT 'FA-',
            quote_prefix TEXT NOT NULL DEFAULT 'DE-',
            next_invoice_number INTEGER NOT NULL DEFAULT 1,
            next_quote_number INTEGER NOT NULL DEFAULT 1,
            legal_mentions TEXT,
            bank_details TEXT,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Insert default company settings if not exists
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO company_settings (id) VALUES ('default')
        "#,
    )
    .execute(pool)
    .await?;

    // Add logo_snapshot column to invoices (for existing databases)
    // This captures the logo at the time of invoice issuance
    sqlx::query(
        r#"
        ALTER TABLE invoices ADD COLUMN logo_snapshot TEXT
        "#,
    )
    .execute(pool)
    .await
    .ok(); // Ignore error if column already exists

    // Add logo_snapshot column to quotes (for existing databases)
    // This captures the logo at the time of quote sending/acceptance
    sqlx::query(
        r#"
        ALTER TABLE quotes ADD COLUMN logo_snapshot TEXT
        "#,
    )
    .execute(pool)
    .await
    .ok(); // Ignore error if column already exists

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quotes_client_id ON quotes(client_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_client_id ON invoices(client_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quote_lines_quote_id ON quote_lines(quote_id)")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_invoice_lines_invoice_id ON invoice_lines(invoice_id)",
    )
    .execute(pool)
    .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_payments_invoice_id ON payments(invoice_id)")
        .execute(pool)
        .await?;

    // Phase 2: Subtotals - Add group_name and is_subtotal_line to quote_lines
    sqlx::query("ALTER TABLE quote_lines ADD COLUMN group_name TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE quote_lines ADD COLUMN is_subtotal_line INTEGER DEFAULT 0")
        .execute(pool)
        .await
        .ok();

    // Phase 2: Subtotals - Add group_name and is_subtotal_line to invoice_lines
    sqlx::query("ALTER TABLE invoice_lines ADD COLUMN group_name TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE invoice_lines ADD COLUMN is_subtotal_line INTEGER DEFAULT 0")
        .execute(pool)
        .await
        .ok();

    // Phase 2: Shipping costs for quotes
    sqlx::query("ALTER TABLE quotes ADD COLUMN shipping_cost_ht REAL DEFAULT 0")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE quotes ADD COLUMN shipping_vat_rate REAL DEFAULT 20")
        .execute(pool)
        .await
        .ok();

    // Phase 2: Shipping costs for invoices
    sqlx::query("ALTER TABLE invoices ADD COLUMN shipping_cost_ht REAL DEFAULT 0")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE invoices ADD COLUMN shipping_vat_rate REAL DEFAULT 20")
        .execute(pool)
        .await
        .ok();

    // Phase 2: Down payments for quotes
    sqlx::query("ALTER TABLE quotes ADD COLUMN down_payment_percent REAL DEFAULT 0")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE quotes ADD COLUMN down_payment_amount REAL DEFAULT 0")
        .execute(pool)
        .await
        .ok();

    // Phase 2: Down payments for invoices
    sqlx::query("ALTER TABLE invoices ADD COLUMN down_payment_percent REAL DEFAULT 0")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE invoices ADD COLUMN down_payment_amount REAL DEFAULT 0")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE invoices ADD COLUMN is_down_payment_invoice INTEGER DEFAULT 0")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE invoices ADD COLUMN parent_quote_id TEXT")
        .execute(pool)
        .await
        .ok();

    // Phase 3: Product categories
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            parent_id TEXT REFERENCES product_categories(id),
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .ok();

    // Phase 3: Add category_id to products
    sqlx::query("ALTER TABLE products ADD COLUMN category_id TEXT REFERENCES product_categories(id)")
        .execute(pool)
        .await
        .ok();

    // Phase 3: Add photo_path to products
    sqlx::query("ALTER TABLE products ADD COLUMN photo_path TEXT")
        .execute(pool)
        .await
        .ok();

    // Phase 4: Delivery Notes
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS delivery_notes (
            id TEXT PRIMARY KEY,
            delivery_note_number TEXT NOT NULL UNIQUE,
            client_id TEXT NOT NULL,
            quote_id TEXT,
            invoice_id TEXT,
            status TEXT NOT NULL DEFAULT 'DRAFT',
            issue_date TEXT NOT NULL,
            delivery_date TEXT,
            delivery_address TEXT,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (client_id) REFERENCES clients(id),
            FOREIGN KEY (quote_id) REFERENCES quotes(id),
            FOREIGN KEY (invoice_id) REFERENCES invoices(id)
        )
        "#,
    )
    .execute(pool)
    .await
    .ok();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS delivery_note_lines (
            id TEXT PRIMARY KEY,
            delivery_note_id TEXT NOT NULL,
            product_id TEXT,
            description TEXT NOT NULL,
            quantity REAL NOT NULL,
            unit TEXT,
            position INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (delivery_note_id) REFERENCES delivery_notes(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        )
        "#,
    )
    .execute(pool)
    .await
    .ok();

    // Add delivery_note_prefix and next_delivery_note_number to company_settings
    sqlx::query("ALTER TABLE company_settings ADD COLUMN delivery_note_prefix TEXT DEFAULT 'BL-'")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE company_settings ADD COLUMN next_delivery_note_number INTEGER DEFAULT 1")
        .execute(pool)
        .await
        .ok();

    // Create indexes for delivery notes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_delivery_notes_client_id ON delivery_notes(client_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_delivery_note_lines_delivery_note_id ON delivery_note_lines(delivery_note_id)")
        .execute(pool)
        .await
        .ok();

    // Phase 5: Client contacts
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS client_contacts (
            id TEXT PRIMARY KEY,
            client_id TEXT NOT NULL,
            name TEXT NOT NULL,
            role TEXT,
            email TEXT,
            phone TEXT,
            is_primary INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await
    .ok();

    // Phase 5: Reminders
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS reminders (
            id TEXT PRIMARY KEY,
            reminder_type TEXT NOT NULL,
            document_type TEXT NOT NULL,
            document_id TEXT NOT NULL,
            scheduled_date TEXT NOT NULL,
            sent_at TEXT,
            message TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await
    .ok();

    // Create indexes for Phase 5
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_client_contacts_client_id ON client_contacts(client_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_reminders_document ON reminders(document_type, document_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_reminders_scheduled_date ON reminders(scheduled_date)")
        .execute(pool)
        .await
        .ok();

    // Phase 7: Rich text - Add description_html columns for quote and invoice lines
    sqlx::query("ALTER TABLE quote_lines ADD COLUMN description_html TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE invoice_lines ADD COLUMN description_html TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE delivery_note_lines ADD COLUMN description_html TEXT")
        .execute(pool)
        .await
        .ok();

    // Phase 7: Rich text - Add notes_html columns for documents
    sqlx::query("ALTER TABLE quotes ADD COLUMN notes_html TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE invoices ADD COLUMN notes_html TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE delivery_notes ADD COLUMN notes_html TEXT")
        .execute(pool)
        .await
        .ok();

    // Phase 7: Rich text - Add description_html to products
    sqlx::query("ALTER TABLE products ADD COLUMN description_html TEXT")
        .execute(pool)
        .await
        .ok();

    // Phase 7: Rich text - Add legal_mentions_html to company_settings
    sqlx::query("ALTER TABLE company_settings ADD COLUMN legal_mentions_html TEXT")
        .execute(pool)
        .await
        .ok();

    // Phase 8: Cloud backup settings
    sqlx::query("ALTER TABLE company_settings ADD COLUMN backup_schedule TEXT DEFAULT 'manual'")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE company_settings ADD COLUMN last_backup_date TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE company_settings ADD COLUMN cloud_provider TEXT")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE company_settings ADD COLUMN auto_backup_enabled INTEGER DEFAULT 0")
        .execute(pool)
        .await
        .ok();

    // Phase 9: Internationalization and theming
    sqlx::query("ALTER TABLE company_settings ADD COLUMN app_language TEXT DEFAULT 'en'")
        .execute(pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE company_settings ADD COLUMN app_theme TEXT DEFAULT 'light'")
        .execute(pool)
        .await
        .ok();

    Ok(())
}
