use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
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
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
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
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create product_categories table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            parent_id TEXT REFERENCES product_categories(id),
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL
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
            amount DOUBLE PRECISION NOT NULL,
            payment_date DATE NOT NULL,
            payment_method TEXT NOT NULL,
            reference TEXT,
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Insert default company settings if not exists
    sqlx::query(
        r#"
        INSERT INTO company_settings (id) VALUES ('default') ON CONFLICT DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    // Create delivery_notes table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create delivery_note_lines table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create client_contacts table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create reminders table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS reminders (
            id TEXT PRIMARY KEY,
            reminder_type TEXT NOT NULL,
            document_type TEXT NOT NULL,
            document_id TEXT NOT NULL,
            scheduled_date DATE NOT NULL,
            sent_at TIMESTAMPTZ,
            message TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create suppliers table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS suppliers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT,
            phone TEXT,
            address TEXT,
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create product_suppliers join table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_suppliers (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            supplier_id TEXT NOT NULL,
            purchase_price_ht DOUBLE PRECISION NOT NULL DEFAULT 0,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (supplier_id) REFERENCES suppliers(id) ON DELETE CASCADE,
            UNIQUE(product_id, supplier_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create expenses table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS expenses (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            amount DOUBLE PRECISION NOT NULL DEFAULT 0,
            date DATE NOT NULL,
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'employee',
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create user_permissions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS user_permissions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            permission_key TEXT NOT NULL,
            granted BOOLEAN NOT NULL DEFAULT TRUE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            UNIQUE(user_id, permission_key)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quotes_client_id ON quotes(client_id)")
        .execute(pool)
        .await
        .ok();
    // Migrate reminders.sent_at from TEXT to TIMESTAMPTZ for existing databases
    sqlx::query("ALTER TABLE reminders ALTER COLUMN sent_at TYPE TIMESTAMPTZ USING sent_at::TIMESTAMPTZ")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_client_id ON invoices(client_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_quote_lines_quote_id ON quote_lines(quote_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_invoice_lines_invoice_id ON invoice_lines(invoice_id)",
    )
    .execute(pool)
    .await
    .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_payments_invoice_id ON payments(invoice_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_delivery_notes_client_id ON delivery_notes(client_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_delivery_note_lines_delivery_note_id ON delivery_note_lines(delivery_note_id)")
        .execute(pool)
        .await
        .ok();
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
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_suppliers_product_id ON product_suppliers(product_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_suppliers_supplier_id ON product_suppliers(supplier_id)")
        .execute(pool)
        .await
        .ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_user_permissions_user_id ON user_permissions(user_id)")
        .execute(pool)
        .await
        .ok();

    Ok(())
}
