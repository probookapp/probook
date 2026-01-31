use chrono::Utc;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;

// Compute SHA-256 hash of invoice immutable fields
fn compute_invoice_hash(invoice: &Invoice) -> String {
    let mut hasher = Sha256::new();

    // Hash immutable fields
    hasher.update(invoice.invoice_number.as_bytes());
    hasher.update(invoice.client_id.as_bytes());
    hasher.update(invoice.issue_date.to_string().as_bytes());
    hasher.update(invoice.due_date.to_string().as_bytes());
    hasher.update(format!("{:.2}", invoice.total_ht).as_bytes());
    hasher.update(format!("{:.2}", invoice.total_vat).as_bytes());
    hasher.update(format!("{:.2}", invoice.total_ttc).as_bytes());

    // Hash line items
    for line in &invoice.lines {
        hasher.update(line.description.as_bytes());
        hasher.update(format!("{:.2}", line.quantity).as_bytes());
        hasher.update(format!("{:.2}", line.unit_price_ht).as_bytes());
        hasher.update(format!("{:.2}", line.vat_rate).as_bytes());
        hasher.update(format!("{:.2}", line.total_ttc).as_bytes());
    }

    let result = hasher.finalize();
    format!("{:x}", result)
}

// Client Repository
pub async fn get_all_clients(pool: &PgPool) -> Result<Vec<Client>, sqlx::Error> {
    sqlx::query_as::<_, Client>("SELECT * FROM clients ORDER BY name")
        .fetch_all(pool)
        .await
}

pub async fn get_client_by_id(pool: &PgPool, id: &str) -> Result<Client, sqlx::Error> {
    sqlx::query_as::<_, Client>("SELECT * FROM clients WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_client(pool: &PgPool, input: CreateClientInput) -> Result<Client, sqlx::Error> {
    let client = Client::new(input);
    sqlx::query(
        r#"
        INSERT INTO clients (id, name, email, phone, address, city, postal_code, country, siret, vat_number, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
    )
    .bind(&client.id)
    .bind(&client.name)
    .bind(&client.email)
    .bind(&client.phone)
    .bind(&client.address)
    .bind(&client.city)
    .bind(&client.postal_code)
    .bind(&client.country)
    .bind(&client.siret)
    .bind(&client.vat_number)
    .bind(&client.notes)
    .bind(&client.created_at)
    .bind(&client.updated_at)
    .execute(pool)
    .await?;

    Ok(client)
}

pub async fn update_client(pool: &PgPool, input: UpdateClientInput) -> Result<Client, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE clients SET name = $1, email = $2, phone = $3, address = $4, city = $5, postal_code = $6, country = $7, siret = $8, vat_number = $9, notes = $10, updated_at = $11
        WHERE id = $12
        "#,
    )
    .bind(&input.name)
    .bind(&input.email)
    .bind(&input.phone)
    .bind(&input.address)
    .bind(&input.city)
    .bind(&input.postal_code)
    .bind(&input.country)
    .bind(&input.siret)
    .bind(&input.vat_number)
    .bind(&input.notes)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    get_client_by_id(pool, &input.id).await
}

pub async fn delete_client(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM clients WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_clients(pool: &PgPool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM clients WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

// Product Repository
pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY designation")
        .fetch_all(pool)
        .await
}

pub async fn get_product_by_id(pool: &PgPool, id: &str) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_product(pool: &PgPool, input: CreateProductInput) -> Result<Product, sqlx::Error> {
    let product = Product::new(input);
    sqlx::query(
        r#"
        INSERT INTO products (id, designation, description, unit_price_ht, vat_rate, unit, reference, is_service, category_id, quantity, purchase_price_ht, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
    )
    .bind(&product.id)
    .bind(&product.designation)
    .bind(&product.description)
    .bind(&product.unit_price_ht)
    .bind(&product.vat_rate)
    .bind(&product.unit)
    .bind(&product.reference)
    .bind(&product.is_service)
    .bind(&product.category_id)
    .bind(&product.quantity)
    .bind(&product.purchase_price_ht)
    .bind(&product.created_at)
    .bind(&product.updated_at)
    .execute(pool)
    .await?;

    Ok(product)
}

pub async fn update_product(pool: &PgPool, input: UpdateProductInput) -> Result<Product, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE products SET designation = $1, description = $2, unit_price_ht = $3, vat_rate = $4, unit = $5, reference = $6, is_service = $7, category_id = $8, quantity = $9, purchase_price_ht = $10, updated_at = $11
        WHERE id = $12
        "#,
    )
    .bind(&input.designation)
    .bind(&input.description)
    .bind(&input.unit_price_ht)
    .bind(&input.vat_rate)
    .bind(&input.unit)
    .bind(&input.reference)
    .bind(&input.is_service)
    .bind(&input.category_id)
    .bind(&input.quantity)
    .bind(&input.purchase_price_ht)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    get_product_by_id(pool, &input.id).await
}

pub async fn delete_product(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_products(pool: &PgPool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

// Decrease product stock by quantity (only for non-service products)
pub async fn decrease_product_stock(pool: &PgPool, product_id: &str, quantity: f64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products SET quantity = GREATEST(0, COALESCE(quantity, 0) - $1), updated_at = $2
        WHERE id = $3 AND is_service = false
        "#,
    )
    .bind(quantity as i32)
    .bind(Utc::now())
    .bind(product_id)
    .execute(pool)
    .await?;
    Ok(())
}

// Quote Repository
pub async fn get_all_quotes(pool: &PgPool) -> Result<Vec<Quote>, sqlx::Error> {
    let rows = sqlx::query_as::<_, QuoteRow>("SELECT * FROM quotes ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    let mut quotes = Vec::new();
    for row in rows {
        let client = get_client_by_id(pool, &row.client_id).await.ok();
        let lines = get_quote_lines(pool, &row.id).await?;
        let status = match row.status.as_str() {
            "SENT" => QuoteStatus::SENT,
            "ACCEPTED" => QuoteStatus::ACCEPTED,
            "EXPIRED" => QuoteStatus::EXPIRED,
            _ => QuoteStatus::DRAFT,
        };
        quotes.push(Quote {
            id: row.id,
            quote_number: row.quote_number,
            client_id: row.client_id,
            client,
            status,
            issue_date: row.issue_date,
            validity_date: row.validity_date,
            total_ht: row.total_ht,
            total_vat: row.total_vat,
            total_ttc: row.total_ttc,
            notes: row.notes,
            notes_html: row.notes_html,
            logo_snapshot: row.logo_snapshot,
            shipping_cost_ht: row.shipping_cost_ht.unwrap_or(0.0),
            shipping_vat_rate: row.shipping_vat_rate.unwrap_or(20.0),
            down_payment_percent: row.down_payment_percent.unwrap_or(0.0),
            down_payment_amount: row.down_payment_amount.unwrap_or(0.0),
            lines,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }
    Ok(quotes)
}

pub async fn get_quote_by_id(pool: &PgPool, id: &str) -> Result<Quote, sqlx::Error> {
    let row = sqlx::query_as::<_, QuoteRow>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let client = get_client_by_id(pool, &row.client_id).await.ok();
    let lines = get_quote_lines(pool, &row.id).await?;
    let status = match row.status.as_str() {
        "SENT" => QuoteStatus::SENT,
        "ACCEPTED" => QuoteStatus::ACCEPTED,
        "EXPIRED" => QuoteStatus::EXPIRED,
        _ => QuoteStatus::DRAFT,
    };

    Ok(Quote {
        id: row.id,
        quote_number: row.quote_number,
        client_id: row.client_id,
        client,
        status,
        issue_date: row.issue_date,
        validity_date: row.validity_date,
        total_ht: row.total_ht,
        total_vat: row.total_vat,
        total_ttc: row.total_ttc,
        notes: row.notes,
        notes_html: row.notes_html,
        logo_snapshot: row.logo_snapshot,
        shipping_cost_ht: row.shipping_cost_ht.unwrap_or(0.0),
        shipping_vat_rate: row.shipping_vat_rate.unwrap_or(20.0),
        down_payment_percent: row.down_payment_percent.unwrap_or(0.0),
        down_payment_amount: row.down_payment_amount.unwrap_or(0.0),
        lines,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

async fn get_quote_lines(pool: &PgPool, quote_id: &str) -> Result<Vec<QuoteLine>, sqlx::Error> {
    sqlx::query_as::<_, QuoteLine>("SELECT * FROM quote_lines WHERE quote_id = $1 ORDER BY position")
        .bind(quote_id)
        .fetch_all(pool)
        .await
}

pub async fn create_quote(pool: &PgPool, input: CreateQuoteInput) -> Result<Quote, sqlx::Error> {
    let settings = get_company_settings(pool).await?;
    let quote_number = format!("{}{}-{:04}", settings.quote_prefix, chrono::Utc::now().format("%Y"), settings.next_quote_number);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // Calculate totals
    let lines: Vec<QuoteLine> = input.lines.iter().enumerate()
        .map(|(i, l)| QuoteLine::new(&id, l.clone(), i as i32))
        .collect();
    let total_ht: f64 = lines.iter().map(|l| l.total_ht).sum();
    let total_vat: f64 = lines.iter().map(|l| l.total_vat).sum();
    let total_ttc: f64 = lines.iter().map(|l| l.total_ttc).sum();

    sqlx::query(
        r#"
        INSERT INTO quotes (id, quote_number, client_id, status, issue_date, validity_date, total_ht, total_vat, total_ttc, notes, created_at, updated_at)
        VALUES ($1, $2, $3, 'DRAFT', $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&id)
    .bind(&quote_number)
    .bind(&input.client_id)
    .bind(&input.issue_date)
    .bind(&input.validity_date)
    .bind(total_ht)
    .bind(total_vat)
    .bind(total_ttc)
    .bind(&input.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // Insert lines
    for line in &lines {
        sqlx::query(
            r#"
            INSERT INTO quote_lines (id, quote_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line.id)
        .bind(&line.quote_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(line.position)
        .execute(pool)
        .await?;
    }

    // Increment quote number
    sqlx::query("UPDATE company_settings SET next_quote_number = next_quote_number + 1 WHERE id = 'default'")
        .execute(pool)
        .await?;

    get_quote_by_id(pool, &id).await
}

pub async fn update_quote(pool: &PgPool, input: UpdateQuoteInput, logo_snapshot: Option<String>) -> Result<Quote, sqlx::Error> {
    let now = Utc::now();

    // Get current quote to check if we need to capture logo
    let current_quote = get_quote_by_id(pool, &input.id).await?;

    // Determine if we should capture the logo snapshot
    // Only capture when transitioning from DRAFT to SENT or ACCEPTED
    let should_capture_logo = current_quote.status == QuoteStatus::DRAFT
        && (input.status == QuoteStatus::SENT || input.status == QuoteStatus::ACCEPTED);

    let final_logo_snapshot = if should_capture_logo {
        logo_snapshot
    } else {
        // Keep existing logo snapshot
        current_quote.logo_snapshot
    };

    // Delete existing lines
    sqlx::query("DELETE FROM quote_lines WHERE quote_id = $1")
        .bind(&input.id)
        .execute(pool)
        .await?;

    // Calculate totals
    let lines: Vec<QuoteLine> = input.lines.iter().enumerate()
        .map(|(i, l)| QuoteLine::new(&input.id, l.clone(), i as i32))
        .collect();
    let total_ht: f64 = lines.iter().map(|l| l.total_ht).sum();
    let total_vat: f64 = lines.iter().map(|l| l.total_vat).sum();
    let total_ttc: f64 = lines.iter().map(|l| l.total_ttc).sum();

    sqlx::query(
        r#"
        UPDATE quotes SET client_id = $1, status = $2, issue_date = $3, validity_date = $4, total_ht = $5, total_vat = $6, total_ttc = $7, notes = $8, logo_snapshot = $9, updated_at = $10
        WHERE id = $11
        "#,
    )
    .bind(&input.client_id)
    .bind(input.status.to_string())
    .bind(&input.issue_date)
    .bind(&input.validity_date)
    .bind(total_ht)
    .bind(total_vat)
    .bind(total_ttc)
    .bind(&input.notes)
    .bind(&final_logo_snapshot)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    // Insert new lines
    for line in &lines {
        sqlx::query(
            r#"
            INSERT INTO quote_lines (id, quote_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line.id)
        .bind(&line.quote_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(line.position)
        .execute(pool)
        .await?;
    }

    get_quote_by_id(pool, &input.id).await
}

pub async fn delete_quote(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM quotes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_quotes(pool: &PgPool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM quotes WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

// Invoice Repository (similar structure to quotes)
pub async fn get_all_invoices(pool: &PgPool) -> Result<Vec<Invoice>, sqlx::Error> {
    let rows = sqlx::query_as::<_, InvoiceRow>("SELECT * FROM invoices ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    let mut invoices = Vec::new();
    for row in rows {
        let client = get_client_by_id(pool, &row.client_id).await.ok();
        let lines = get_invoice_lines(pool, &row.id).await?;
        let payments = get_payments_by_invoice(pool, &row.id).await?;
        let status = match row.status.as_str() {
            "ISSUED" => InvoiceStatus::ISSUED,
            "PAID" => InvoiceStatus::PAID,
            _ => InvoiceStatus::DRAFT,
        };
        invoices.push(Invoice {
            id: row.id,
            invoice_number: row.invoice_number,
            client_id: row.client_id,
            client,
            quote_id: row.quote_id,
            status,
            issue_date: row.issue_date,
            due_date: row.due_date,
            total_ht: row.total_ht,
            total_vat: row.total_vat,
            total_ttc: row.total_ttc,
            notes: row.notes,
            notes_html: row.notes_html,
            integrity_hash: row.integrity_hash,
            logo_snapshot: row.logo_snapshot,
            shipping_cost_ht: row.shipping_cost_ht.unwrap_or(0.0),
            shipping_vat_rate: row.shipping_vat_rate.unwrap_or(20.0),
            down_payment_percent: row.down_payment_percent.unwrap_or(0.0),
            down_payment_amount: row.down_payment_amount.unwrap_or(0.0),
            is_down_payment_invoice: row.is_down_payment_invoice.unwrap_or(false),
            parent_quote_id: row.parent_quote_id,
            lines,
            payments,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }
    Ok(invoices)
}

pub async fn get_invoice_by_id(pool: &PgPool, id: &str) -> Result<Invoice, sqlx::Error> {
    let row = sqlx::query_as::<_, InvoiceRow>("SELECT * FROM invoices WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let client = get_client_by_id(pool, &row.client_id).await.ok();
    let lines = get_invoice_lines(pool, &row.id).await?;
    let payments = get_payments_by_invoice(pool, &row.id).await?;
    let status = match row.status.as_str() {
        "ISSUED" => InvoiceStatus::ISSUED,
        "PAID" => InvoiceStatus::PAID,
        _ => InvoiceStatus::DRAFT,
    };

    Ok(Invoice {
        id: row.id,
        invoice_number: row.invoice_number,
        client_id: row.client_id,
        client,
        quote_id: row.quote_id,
        status,
        issue_date: row.issue_date,
        due_date: row.due_date,
        total_ht: row.total_ht,
        total_vat: row.total_vat,
        total_ttc: row.total_ttc,
        notes: row.notes,
        notes_html: row.notes_html,
        integrity_hash: row.integrity_hash,
        logo_snapshot: row.logo_snapshot,
        shipping_cost_ht: row.shipping_cost_ht.unwrap_or(0.0),
        shipping_vat_rate: row.shipping_vat_rate.unwrap_or(20.0),
        down_payment_percent: row.down_payment_percent.unwrap_or(0.0),
        down_payment_amount: row.down_payment_amount.unwrap_or(0.0),
        is_down_payment_invoice: row.is_down_payment_invoice.unwrap_or(false),
        parent_quote_id: row.parent_quote_id,
        lines,
        payments,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

async fn get_invoice_lines(pool: &PgPool, invoice_id: &str) -> Result<Vec<InvoiceLine>, sqlx::Error> {
    sqlx::query_as::<_, InvoiceLine>("SELECT * FROM invoice_lines WHERE invoice_id = $1 ORDER BY position")
        .bind(invoice_id)
        .fetch_all(pool)
        .await
}

pub async fn create_invoice(pool: &PgPool, input: CreateInvoiceInput) -> Result<Invoice, sqlx::Error> {
    create_invoice_internal(pool, input, true).await
}

async fn create_invoice_internal(pool: &PgPool, input: CreateInvoiceInput, decrease_stock: bool) -> Result<Invoice, sqlx::Error> {
    let settings = get_company_settings(pool).await?;
    let invoice_number = format!("{}{}-{:04}", settings.invoice_prefix, chrono::Utc::now().format("%Y"), settings.next_invoice_number);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // Calculate totals
    let lines: Vec<InvoiceLine> = input.lines.iter().enumerate()
        .map(|(i, l)| InvoiceLine::new(&id, l.clone(), i as i32))
        .collect();
    let total_ht: f64 = lines.iter().map(|l| l.total_ht).sum();
    let total_vat: f64 = lines.iter().map(|l| l.total_vat).sum();
    let total_ttc: f64 = lines.iter().map(|l| l.total_ttc).sum();

    sqlx::query(
        r#"
        INSERT INTO invoices (id, invoice_number, client_id, quote_id, status, issue_date, due_date, total_ht, total_vat, total_ttc, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'DRAFT', $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
    )
    .bind(&id)
    .bind(&invoice_number)
    .bind(&input.client_id)
    .bind(&input.quote_id)
    .bind(&input.issue_date)
    .bind(&input.due_date)
    .bind(total_ht)
    .bind(total_vat)
    .bind(total_ttc)
    .bind(&input.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // Insert lines
    for line in &lines {
        sqlx::query(
            r#"
            INSERT INTO invoice_lines (id, invoice_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line.id)
        .bind(&line.invoice_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(line.position)
        .execute(pool)
        .await?;
    }

    // Decrease stock for product lines (skip when converting from delivery notes to avoid double-deduction)
    if decrease_stock {
        for line in &lines {
            if let Some(ref product_id) = line.product_id {
                decrease_product_stock(pool, product_id, line.quantity).await?;
            }
        }
    }

    // Increment invoice number
    sqlx::query("UPDATE company_settings SET next_invoice_number = next_invoice_number + 1 WHERE id = 'default'")
        .execute(pool)
        .await?;

    get_invoice_by_id(pool, &id).await
}

pub async fn update_invoice(pool: &PgPool, input: UpdateInvoiceInput) -> Result<Invoice, sqlx::Error> {
    let now = Utc::now();

    // Delete existing lines
    sqlx::query("DELETE FROM invoice_lines WHERE invoice_id = $1")
        .bind(&input.id)
        .execute(pool)
        .await?;

    // Calculate totals
    let lines: Vec<InvoiceLine> = input.lines.iter().enumerate()
        .map(|(i, l)| InvoiceLine::new(&input.id, l.clone(), i as i32))
        .collect();
    let total_ht: f64 = lines.iter().map(|l| l.total_ht).sum();
    let total_vat: f64 = lines.iter().map(|l| l.total_vat).sum();
    let total_ttc: f64 = lines.iter().map(|l| l.total_ttc).sum();

    sqlx::query(
        r#"
        UPDATE invoices SET client_id = $1, status = $2, issue_date = $3, due_date = $4, total_ht = $5, total_vat = $6, total_ttc = $7, notes = $8, updated_at = $9
        WHERE id = $10
        "#,
    )
    .bind(&input.client_id)
    .bind(input.status.to_string())
    .bind(&input.issue_date)
    .bind(&input.due_date)
    .bind(total_ht)
    .bind(total_vat)
    .bind(total_ttc)
    .bind(&input.notes)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    // Insert new lines
    for line in &lines {
        sqlx::query(
            r#"
            INSERT INTO invoice_lines (id, invoice_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line.id)
        .bind(&line.invoice_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(line.position)
        .execute(pool)
        .await?;
    }

    get_invoice_by_id(pool, &input.id).await
}

pub async fn delete_invoice(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM invoices WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_invoices(pool: &PgPool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        let row: (String,) = sqlx::query_as("SELECT status FROM invoices WHERE id = $1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;
        if row.0 != "DRAFT" {
            return Err(sqlx::Error::Protocol(
                format!("Cannot delete non-DRAFT invoice {}", id),
            ));
        }
        sqlx::query("DELETE FROM invoices WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn mark_invoice_paid(pool: &PgPool, id: &str) -> Result<Invoice, sqlx::Error> {
    let now = Utc::now();

    // First get the invoice to compute hash if not already set
    let invoice = get_invoice_by_id(pool, id).await?;

    // If no integrity hash, compute and set it
    if invoice.integrity_hash.is_none() {
        let hash = compute_invoice_hash(&invoice);
        sqlx::query("UPDATE invoices SET status = 'PAID', integrity_hash = $1, updated_at = $2 WHERE id = $3")
            .bind(&hash)
            .bind(&now)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE invoices SET status = 'PAID', updated_at = $1 WHERE id = $2")
            .bind(&now)
            .bind(id)
            .execute(pool)
            .await?;
    }

    get_invoice_by_id(pool, id).await
}

pub async fn issue_invoice(pool: &PgPool, id: &str, logo_snapshot: Option<String>) -> Result<Invoice, sqlx::Error> {
    let now = Utc::now();

    // Get the invoice and compute integrity hash
    let invoice = get_invoice_by_id(pool, id).await?;
    let hash = compute_invoice_hash(&invoice);

    sqlx::query("UPDATE invoices SET status = 'ISSUED', integrity_hash = $1, logo_snapshot = $2, updated_at = $3 WHERE id = $4")
        .bind(&hash)
        .bind(&logo_snapshot)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

    get_invoice_by_id(pool, id).await
}

pub async fn verify_invoice_integrity(pool: &PgPool, id: &str) -> Result<bool, sqlx::Error> {
    let invoice = get_invoice_by_id(pool, id).await?;

    if let Some(stored_hash) = &invoice.integrity_hash {
        let computed_hash = compute_invoice_hash(&invoice);
        Ok(stored_hash == &computed_hash)
    } else {
        // No hash stored, invoice hasn't been issued yet
        Ok(true)
    }
}

// Payment Repository
pub async fn get_all_payments(pool: &PgPool) -> Result<Vec<Payment>, sqlx::Error> {
    sqlx::query_as::<_, Payment>("SELECT * FROM payments ORDER BY payment_date DESC")
        .fetch_all(pool)
        .await
}

pub async fn get_payments_by_invoice(pool: &PgPool, invoice_id: &str) -> Result<Vec<Payment>, sqlx::Error> {
    sqlx::query_as::<_, Payment>("SELECT * FROM payments WHERE invoice_id = $1 ORDER BY payment_date DESC")
        .bind(invoice_id)
        .fetch_all(pool)
        .await
}

pub async fn create_payment(pool: &PgPool, input: CreatePaymentInput) -> Result<Payment, sqlx::Error> {
    let invoice_id = input.invoice_id.clone();
    let payment = Payment::new(input);
    sqlx::query(
        r#"
        INSERT INTO payments (id, invoice_id, amount, payment_date, payment_method, reference, notes, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&payment.id)
    .bind(&payment.invoice_id)
    .bind(payment.amount)
    .bind(&payment.payment_date)
    .bind(&payment.payment_method)
    .bind(&payment.reference)
    .bind(&payment.notes)
    .bind(&payment.created_at)
    .execute(pool)
    .await?;

    // Check if invoice is fully paid and auto-mark as PAID
    let invoice = get_invoice_by_id(pool, &invoice_id).await?;
    let total_paid: f64 = invoice.payments.iter().map(|p| p.amount).sum();

    // Use a small epsilon for floating point comparison
    if total_paid >= invoice.total_ttc - 0.01 && invoice.status != InvoiceStatus::PAID {
        mark_invoice_paid(pool, &invoice_id).await?;
    }

    Ok(payment)
}

pub async fn delete_payment(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM payments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Company Settings Repository
pub async fn get_company_settings(pool: &PgPool) -> Result<CompanySettings, sqlx::Error> {
    sqlx::query_as::<_, CompanySettings>("SELECT * FROM company_settings WHERE id = 'default'")
        .fetch_one(pool)
        .await
}

pub async fn update_company_settings(pool: &PgPool, input: UpdateCompanySettingsInput) -> Result<CompanySettings, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE company_settings SET
            company_name = $1, address = $2, city = $3, postal_code = $4, country = $5,
            phone = $6, email = $7, website = $8, siret = $9, vat_number = $10,
            default_vat_rate = $11, default_payment_terms = $12, invoice_prefix = $13, quote_prefix = $14,
            legal_mentions = $15, bank_details = $16, currency = $17, updated_at = $18
        WHERE id = 'default'
        "#,
    )
    .bind(&input.company_name)
    .bind(&input.address)
    .bind(&input.city)
    .bind(&input.postal_code)
    .bind(&input.country)
    .bind(&input.phone)
    .bind(&input.email)
    .bind(&input.website)
    .bind(&input.siret)
    .bind(&input.vat_number)
    .bind(input.default_vat_rate)
    .bind(input.default_payment_terms)
    .bind(&input.invoice_prefix)
    .bind(&input.quote_prefix)
    .bind(&input.legal_mentions)
    .bind(&input.bank_details)
    .bind(&input.currency)
    .bind(&now)
    .execute(pool)
    .await?;

    get_company_settings(pool).await
}

pub async fn update_last_backup_date(pool: &PgPool) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE company_settings SET last_backup_date = $1 WHERE id = 'default'")
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_app_settings(
    pool: &PgPool,
    app_language: &str,
    app_theme: &str,
    auto_update_enabled: bool,
) -> Result<CompanySettings, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE company_settings SET
            app_language = $1,
            app_theme = $2,
            auto_update_enabled = $3,
            updated_at = $4
        WHERE id = 'default'
        "#,
    )
    .bind(app_language)
    .bind(app_theme)
    .bind(auto_update_enabled)
    .bind(&now)
    .execute(pool)
    .await?;

    get_company_settings(pool).await
}

// Dashboard Stats
pub async fn get_dashboard_stats(pool: &PgPool) -> Result<DashboardStats, sqlx::Error> {
    let total_clients: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM clients")
        .fetch_one(pool)
        .await?;

    let total_invoices: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices")
        .fetch_one(pool)
        .await?;

    let total_quotes: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM quotes")
        .fetch_one(pool)
        .await?;

    let revenue_this_month: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total_ttc), 0.0) FROM invoices WHERE status = 'PAID' AND to_char(issue_date, 'YYYY-MM') = to_char(CURRENT_TIMESTAMP, 'YYYY-MM')"
    )
    .fetch_one(pool)
    .await?;

    let revenue_this_year: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total_ttc), 0.0) FROM invoices WHERE status = 'PAID' AND to_char(issue_date, 'YYYY') = to_char(CURRENT_TIMESTAMP, 'YYYY')"
    )
    .fetch_one(pool)
    .await?;

    let pending_payments: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total_ttc), 0.0) FROM invoices WHERE status = 'ISSUED'"
    )
    .fetch_one(pool)
    .await?;

    let total_expenses: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount), 0.0) FROM expenses WHERE to_char(date, 'YYYY') = to_char(CURRENT_TIMESTAMP, 'YYYY')"
    )
    .fetch_one(pool)
    .await?;

    let profit = revenue_this_year - total_expenses;

    let recent_invoices = get_all_invoices(pool).await?.into_iter().take(5).collect();
    let recent_quotes = get_all_quotes(pool).await?.into_iter().take(5).collect();

    Ok(DashboardStats {
        total_clients,
        total_invoices,
        total_quotes,
        revenue_this_month,
        revenue_this_year,
        pending_payments,
        total_expenses,
        profit,
        recent_invoices,
        recent_quotes,
    })
}

// Duplicate Quote
pub async fn duplicate_quote(pool: &PgPool, quote_id: &str) -> Result<Quote, sqlx::Error> {
    let original = get_quote_by_id(pool, quote_id).await?;
    let settings = get_company_settings(pool).await?;

    let quote_number = format!(
        "{}{}-{:04}",
        settings.quote_prefix,
        chrono::Utc::now().format("%Y"),
        settings.next_quote_number
    );

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let today = chrono::Utc::now().date_naive();
    let validity = today + chrono::Duration::days(30);

    sqlx::query(
        r#"
        INSERT INTO quotes (id, quote_number, client_id, status, issue_date, validity_date, total_ht, total_vat, total_ttc, notes, created_at, updated_at)
        VALUES ($1, $2, $3, 'DRAFT', $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&id)
    .bind(&quote_number)
    .bind(&original.client_id)
    .bind(today)
    .bind(validity)
    .bind(original.total_ht)
    .bind(original.total_vat)
    .bind(original.total_ttc)
    .bind(&original.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // Copy lines with new IDs
    for (i, line) in original.lines.iter().enumerate() {
        let line_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO quote_lines (id, quote_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line_id)
        .bind(&id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(i as i32)
        .execute(pool)
        .await?;
    }

    // Increment quote number
    sqlx::query("UPDATE company_settings SET next_quote_number = next_quote_number + 1 WHERE id = 'default'")
        .execute(pool)
        .await?;

    get_quote_by_id(pool, &id).await
}

// Duplicate Invoice
pub async fn duplicate_invoice(pool: &PgPool, invoice_id: &str) -> Result<Invoice, sqlx::Error> {
    let original = get_invoice_by_id(pool, invoice_id).await?;
    let settings = get_company_settings(pool).await?;

    let invoice_number = format!(
        "{}{}-{:04}",
        settings.invoice_prefix,
        chrono::Utc::now().format("%Y"),
        settings.next_invoice_number
    );

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let today = chrono::Utc::now().date_naive();
    let due_date = today + chrono::Duration::days(settings.default_payment_terms as i64);

    sqlx::query(
        r#"
        INSERT INTO invoices (id, invoice_number, client_id, quote_id, status, issue_date, due_date, total_ht, total_vat, total_ttc, notes, created_at, updated_at)
        VALUES ($1, $2, $3, NULL, 'DRAFT', $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&id)
    .bind(&invoice_number)
    .bind(&original.client_id)
    .bind(today)
    .bind(due_date)
    .bind(original.total_ht)
    .bind(original.total_vat)
    .bind(original.total_ttc)
    .bind(&original.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // Copy lines with new IDs
    for (i, line) in original.lines.iter().enumerate() {
        let line_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO invoice_lines (id, invoice_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line_id)
        .bind(&id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(i as i32)
        .execute(pool)
        .await?;
    }

    // Increment invoice number
    sqlx::query("UPDATE company_settings SET next_invoice_number = next_invoice_number + 1 WHERE id = 'default'")
        .execute(pool)
        .await?;

    get_invoice_by_id(pool, &id).await
}

// Quote to Invoice conversion
pub async fn convert_quote_to_invoice(pool: &PgPool, quote_id: &str) -> Result<Invoice, sqlx::Error> {
    let quote = get_quote_by_id(pool, quote_id).await?;
    let settings = get_company_settings(pool).await?;

    let invoice_input = CreateInvoiceInput {
        client_id: quote.client_id,
        quote_id: Some(quote_id.to_string()),
        issue_date: chrono::Utc::now().date_naive(),
        due_date: chrono::Utc::now().date_naive() + chrono::Duration::days(settings.default_payment_terms as i64),
        notes: quote.notes,
        notes_html: quote.notes_html,
        shipping_cost_ht: if quote.shipping_cost_ht > 0.0 { Some(quote.shipping_cost_ht) } else { None },
        shipping_vat_rate: Some(quote.shipping_vat_rate),
        down_payment_percent: if quote.down_payment_percent > 0.0 { Some(quote.down_payment_percent) } else { None },
        down_payment_amount: if quote.down_payment_amount > 0.0 { Some(quote.down_payment_amount) } else { None },
        lines: quote.lines.into_iter().map(|l| CreateInvoiceLineInput {
            product_id: l.product_id,
            description: l.description,
            description_html: l.description_html,
            quantity: l.quantity,
            unit_price_ht: l.unit_price_ht,
            vat_rate: l.vat_rate,
            group_name: l.group_name,
            is_subtotal_line: l.is_subtotal_line,
        }).collect(),
    };

    create_invoice(pool, invoice_input).await
}

// Logo Management
pub async fn update_logo_path(pool: &PgPool, logo_path: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    let path = if logo_path.is_empty() {
        None
    } else {
        Some(logo_path.to_string())
    };
    sqlx::query("UPDATE company_settings SET logo_path = $1, updated_at = $2 WHERE id = 'default'")
        .bind(path)
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(())
}

// Backup Restore Functions
pub async fn clear_all_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM payments").execute(pool).await?;
    sqlx::query("DELETE FROM invoice_lines").execute(pool).await?;
    sqlx::query("DELETE FROM invoices").execute(pool).await?;
    sqlx::query("DELETE FROM quote_lines").execute(pool).await?;
    sqlx::query("DELETE FROM quotes").execute(pool).await?;
    sqlx::query("DELETE FROM expenses").execute(pool).await?;
    sqlx::query("DELETE FROM product_suppliers").execute(pool).await?;
    sqlx::query("DELETE FROM products").execute(pool).await?;
    sqlx::query("DELETE FROM suppliers").execute(pool).await?;
    sqlx::query("DELETE FROM clients").execute(pool).await?;
    sqlx::query("DELETE FROM user_permissions").execute(pool).await?;
    sqlx::query("DELETE FROM users").execute(pool).await?;
    Ok(())
}

pub async fn restore_client(pool: &PgPool, client: Client) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO clients (id, name, email, phone, address, city, postal_code, country, siret, vat_number, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
    )
    .bind(&client.id)
    .bind(&client.name)
    .bind(&client.email)
    .bind(&client.phone)
    .bind(&client.address)
    .bind(&client.city)
    .bind(&client.postal_code)
    .bind(&client.country)
    .bind(&client.siret)
    .bind(&client.vat_number)
    .bind(&client.notes)
    .bind(&client.created_at)
    .bind(&client.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn restore_product(pool: &PgPool, product: Product) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO products (id, designation, description, unit_price_ht, vat_rate, unit, reference, is_service, category_id, quantity, purchase_price_ht, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
    )
    .bind(&product.id)
    .bind(&product.designation)
    .bind(&product.description)
    .bind(&product.unit_price_ht)
    .bind(&product.vat_rate)
    .bind(&product.unit)
    .bind(&product.reference)
    .bind(&product.is_service)
    .bind(&product.category_id)
    .bind(&product.quantity)
    .bind(&product.purchase_price_ht)
    .bind(&product.created_at)
    .bind(&product.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn restore_quote(pool: &PgPool, quote: Quote) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO quotes (id, quote_number, client_id, status, issue_date, validity_date, total_ht, total_vat, total_ttc, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
    )
    .bind(&quote.id)
    .bind(&quote.quote_number)
    .bind(&quote.client_id)
    .bind(quote.status.to_string())
    .bind(&quote.issue_date)
    .bind(&quote.validity_date)
    .bind(quote.total_ht)
    .bind(quote.total_vat)
    .bind(quote.total_ttc)
    .bind(&quote.notes)
    .bind(&quote.created_at)
    .bind(&quote.updated_at)
    .execute(pool)
    .await?;

    // Insert quote lines
    for line in quote.lines {
        sqlx::query(
            r#"
            INSERT INTO quote_lines (id, quote_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line.id)
        .bind(&line.quote_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(line.position)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn restore_invoice(pool: &PgPool, invoice: Invoice) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO invoices (id, invoice_number, client_id, quote_id, status, issue_date, due_date, total_ht, total_vat, total_ttc, notes, integrity_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#,
    )
    .bind(&invoice.id)
    .bind(&invoice.invoice_number)
    .bind(&invoice.client_id)
    .bind(&invoice.quote_id)
    .bind(invoice.status.to_string())
    .bind(&invoice.issue_date)
    .bind(&invoice.due_date)
    .bind(invoice.total_ht)
    .bind(invoice.total_vat)
    .bind(invoice.total_ttc)
    .bind(&invoice.notes)
    .bind(&invoice.integrity_hash)
    .bind(&invoice.created_at)
    .bind(&invoice.updated_at)
    .execute(pool)
    .await?;

    // Insert invoice lines
    for line in invoice.lines {
        sqlx::query(
            r#"
            INSERT INTO invoice_lines (id, invoice_id, product_id, description, quantity, unit_price_ht, vat_rate, total_ht, total_vat, total_ttc, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(&line.id)
        .bind(&line.invoice_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price_ht)
        .bind(line.vat_rate)
        .bind(line.total_ht)
        .bind(line.total_vat)
        .bind(line.total_ttc)
        .bind(line.position)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn restore_payment(pool: &PgPool, payment: Payment) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO payments (id, invoice_id, amount, payment_date, payment_method, reference, notes, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&payment.id)
    .bind(&payment.invoice_id)
    .bind(payment.amount)
    .bind(&payment.payment_date)
    .bind(&payment.payment_method)
    .bind(&payment.reference)
    .bind(&payment.notes)
    .bind(&payment.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn restore_settings(pool: &PgPool, settings: CompanySettings) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE company_settings SET
            company_name = $1, address = $2, city = $3, postal_code = $4, country = $5,
            phone = $6, email = $7, website = $8, siret = $9, vat_number = $10, logo_path = $11,
            default_vat_rate = $12, default_payment_terms = $13, invoice_prefix = $14, quote_prefix = $15,
            next_invoice_number = $16, next_quote_number = $17, legal_mentions = $18, bank_details = $19,
            currency = $20, updated_at = $21
        WHERE id = 'default'
        "#,
    )
    .bind(&settings.company_name)
    .bind(&settings.address)
    .bind(&settings.city)
    .bind(&settings.postal_code)
    .bind(&settings.country)
    .bind(&settings.phone)
    .bind(&settings.email)
    .bind(&settings.website)
    .bind(&settings.siret)
    .bind(&settings.vat_number)
    .bind(&settings.logo_path)
    .bind(settings.default_vat_rate)
    .bind(settings.default_payment_terms)
    .bind(&settings.invoice_prefix)
    .bind(&settings.quote_prefix)
    .bind(settings.next_invoice_number)
    .bind(settings.next_quote_number)
    .bind(&settings.legal_mentions)
    .bind(&settings.bank_details)
    .bind(&settings.currency)
    .bind(&settings.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

// Product Category Repository
pub async fn get_all_product_categories(pool: &PgPool) -> Result<Vec<ProductCategory>, sqlx::Error> {
    sqlx::query_as::<_, ProductCategory>("SELECT * FROM product_categories ORDER BY name")
        .fetch_all(pool)
        .await
}

pub async fn get_product_category_by_id(pool: &PgPool, id: &str) -> Result<ProductCategory, sqlx::Error> {
    sqlx::query_as::<_, ProductCategory>("SELECT * FROM product_categories WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_product_category(pool: &PgPool, input: CreateProductCategoryInput) -> Result<ProductCategory, sqlx::Error> {
    let category = ProductCategory::new(input);
    sqlx::query(
        r#"
        INSERT INTO product_categories (id, name, description, parent_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(&category.id)
    .bind(&category.name)
    .bind(&category.description)
    .bind(&category.parent_id)
    .bind(&category.created_at)
    .bind(&category.updated_at)
    .execute(pool)
    .await?;

    Ok(category)
}

pub async fn update_product_category(pool: &PgPool, input: UpdateProductCategoryInput) -> Result<ProductCategory, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE product_categories SET name = $1, description = $2, parent_id = $3, updated_at = $4
        WHERE id = $5
        "#,
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.parent_id)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    get_product_category_by_id(pool, &input.id).await
}

pub async fn delete_product_category(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    // First, unset category_id for all products in this category
    sqlx::query("UPDATE products SET category_id = NULL WHERE category_id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    // Then delete the category
    sqlx::query("DELETE FROM product_categories WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Update product photo path
pub async fn update_product_photo(pool: &PgPool, product_id: &str, photo_path: Option<&str>) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query("UPDATE products SET photo_path = $1, updated_at = $2 WHERE id = $3")
        .bind(photo_path)
        .bind(&now)
        .bind(product_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Delivery Note Repository
pub async fn get_all_delivery_notes(pool: &PgPool) -> Result<Vec<DeliveryNote>, sqlx::Error> {
    let rows = sqlx::query_as::<_, DeliveryNoteRow>("SELECT * FROM delivery_notes ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    let mut delivery_notes = Vec::new();
    for row in rows {
        let client = get_client_by_id(pool, &row.client_id).await.ok();
        let lines = get_delivery_note_lines(pool, &row.id).await?;
        let status = match row.status.as_str() {
            "DELIVERED" => DeliveryNoteStatus::DELIVERED,
            "CANCELLED" => DeliveryNoteStatus::CANCELLED,
            _ => DeliveryNoteStatus::DRAFT,
        };
        delivery_notes.push(DeliveryNote {
            id: row.id,
            delivery_note_number: row.delivery_note_number,
            client_id: row.client_id,
            client,
            quote_id: row.quote_id,
            invoice_id: row.invoice_id,
            status,
            issue_date: row.issue_date,
            delivery_date: row.delivery_date,
            delivery_address: row.delivery_address,
            notes: row.notes,
            lines,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }
    Ok(delivery_notes)
}

pub async fn get_delivery_note_by_id(pool: &PgPool, id: &str) -> Result<DeliveryNote, sqlx::Error> {
    let row = sqlx::query_as::<_, DeliveryNoteRow>("SELECT * FROM delivery_notes WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let client = get_client_by_id(pool, &row.client_id).await.ok();
    let lines = get_delivery_note_lines(pool, &row.id).await?;
    let status = match row.status.as_str() {
        "DELIVERED" => DeliveryNoteStatus::DELIVERED,
        "CANCELLED" => DeliveryNoteStatus::CANCELLED,
        _ => DeliveryNoteStatus::DRAFT,
    };

    Ok(DeliveryNote {
        id: row.id,
        delivery_note_number: row.delivery_note_number,
        client_id: row.client_id,
        client,
        quote_id: row.quote_id,
        invoice_id: row.invoice_id,
        status,
        issue_date: row.issue_date,
        delivery_date: row.delivery_date,
        delivery_address: row.delivery_address,
        notes: row.notes,
        lines,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

async fn get_delivery_note_lines(pool: &PgPool, delivery_note_id: &str) -> Result<Vec<DeliveryNoteLine>, sqlx::Error> {
    sqlx::query_as::<_, DeliveryNoteLine>("SELECT * FROM delivery_note_lines WHERE delivery_note_id = $1 ORDER BY position")
        .bind(delivery_note_id)
        .fetch_all(pool)
        .await
}

pub async fn create_delivery_note(pool: &PgPool, input: CreateDeliveryNoteInput) -> Result<DeliveryNote, sqlx::Error> {
    let settings = get_company_settings(pool).await?;
    let prefix = settings.delivery_note_prefix.unwrap_or_else(|| "BL-".to_string());
    let next_num = settings.next_delivery_note_number.unwrap_or(1);
    let delivery_note_number = format!("{}{}-{:04}", prefix, chrono::Utc::now().format("%Y"), next_num);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let lines: Vec<DeliveryNoteLine> = input.lines.iter().enumerate()
        .map(|(i, l)| DeliveryNoteLine::new(&id, l.clone(), i as i32))
        .collect();

    sqlx::query(
        r#"
        INSERT INTO delivery_notes (id, delivery_note_number, client_id, quote_id, invoice_id, status, issue_date, delivery_date, delivery_address, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&id)
    .bind(&delivery_note_number)
    .bind(&input.client_id)
    .bind(&input.quote_id)
    .bind(&input.invoice_id)
    .bind(&input.issue_date)
    .bind(&input.delivery_date)
    .bind(&input.delivery_address)
    .bind(&input.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // Insert lines
    for line in &lines {
        sqlx::query(
            r#"
            INSERT INTO delivery_note_lines (id, delivery_note_id, product_id, description, quantity, unit, position, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&line.id)
        .bind(&line.delivery_note_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(&line.unit)
        .bind(line.position)
        .bind(&line.created_at)
        .execute(pool)
        .await?;
    }

    // Decrease stock for product lines
    for line in &lines {
        if let Some(ref product_id) = line.product_id {
            decrease_product_stock(pool, product_id, line.quantity).await?;
        }
    }

    // Increment delivery note number
    sqlx::query("UPDATE company_settings SET next_delivery_note_number = COALESCE(next_delivery_note_number, 1) + 1 WHERE id = 'default'")
        .execute(pool)
        .await?;

    get_delivery_note_by_id(pool, &id).await
}

pub async fn update_delivery_note(pool: &PgPool, input: UpdateDeliveryNoteInput) -> Result<DeliveryNote, sqlx::Error> {
    let now = Utc::now();

    // Delete existing lines
    sqlx::query("DELETE FROM delivery_note_lines WHERE delivery_note_id = $1")
        .bind(&input.id)
        .execute(pool)
        .await?;

    let lines: Vec<DeliveryNoteLine> = input.lines.iter().enumerate()
        .map(|(i, l)| DeliveryNoteLine::new(&input.id, l.clone(), i as i32))
        .collect();

    sqlx::query(
        r#"
        UPDATE delivery_notes SET client_id = $1, quote_id = $2, invoice_id = $3, status = $4, issue_date = $5, delivery_date = $6, delivery_address = $7, notes = $8, updated_at = $9
        WHERE id = $10
        "#,
    )
    .bind(&input.client_id)
    .bind(&input.quote_id)
    .bind(&input.invoice_id)
    .bind(input.status.to_string())
    .bind(&input.issue_date)
    .bind(&input.delivery_date)
    .bind(&input.delivery_address)
    .bind(&input.notes)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    // Insert new lines
    for line in &lines {
        sqlx::query(
            r#"
            INSERT INTO delivery_note_lines (id, delivery_note_id, product_id, description, quantity, unit, position, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&line.id)
        .bind(&line.delivery_note_id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(&line.unit)
        .bind(line.position)
        .bind(&line.created_at)
        .execute(pool)
        .await?;
    }

    get_delivery_note_by_id(pool, &input.id).await
}

pub async fn delete_delivery_note(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM delivery_notes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_delivery_notes(pool: &PgPool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM delivery_notes WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn duplicate_delivery_note(pool: &PgPool, delivery_note_id: &str) -> Result<DeliveryNote, sqlx::Error> {
    let original = get_delivery_note_by_id(pool, delivery_note_id).await?;
    let settings = get_company_settings(pool).await?;

    let prefix = settings.delivery_note_prefix.unwrap_or_else(|| "BL-".to_string());
    let next_num = settings.next_delivery_note_number.unwrap_or(1);
    let delivery_note_number = format!("{}{}-{:04}", prefix, chrono::Utc::now().format("%Y"), next_num);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let today = chrono::Utc::now().date_naive();

    sqlx::query(
        r#"
        INSERT INTO delivery_notes (id, delivery_note_number, client_id, quote_id, invoice_id, status, issue_date, delivery_date, delivery_address, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&id)
    .bind(&delivery_note_number)
    .bind(&original.client_id)
    .bind(&original.quote_id)
    .bind(&original.invoice_id)
    .bind(today)
    .bind(&original.delivery_date)
    .bind(&original.delivery_address)
    .bind(&original.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // Copy lines with new IDs
    for (i, line) in original.lines.iter().enumerate() {
        let line_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO delivery_note_lines (id, delivery_note_id, product_id, description, quantity, unit, position, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&line_id)
        .bind(&id)
        .bind(&line.product_id)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(&line.unit)
        .bind(i as i32)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    // Increment delivery note number
    sqlx::query("UPDATE company_settings SET next_delivery_note_number = COALESCE(next_delivery_note_number, 1) + 1 WHERE id = 'default'")
        .execute(pool)
        .await?;

    get_delivery_note_by_id(pool, &id).await
}

// Convert Quote to Delivery Note
pub async fn convert_quote_to_delivery_note(pool: &PgPool, quote_id: &str) -> Result<DeliveryNote, sqlx::Error> {
    let quote = get_quote_by_id(pool, quote_id).await?;

    let delivery_note_input = CreateDeliveryNoteInput {
        client_id: quote.client_id,
        quote_id: Some(quote_id.to_string()),
        invoice_id: None,
        issue_date: chrono::Utc::now().date_naive(),
        delivery_date: None,
        delivery_address: None,
        notes: quote.notes,
        lines: quote.lines.into_iter().map(|l| CreateDeliveryNoteLineInput {
            product_id: l.product_id,
            description: l.description,
            quantity: l.quantity,
            unit: Some("unité".to_string()),
        }).collect(),
    };

    create_delivery_note(pool, delivery_note_input).await
}

// Convert Invoice to Delivery Note
pub async fn convert_invoice_to_delivery_note(pool: &PgPool, invoice_id: &str) -> Result<DeliveryNote, sqlx::Error> {
    let invoice = get_invoice_by_id(pool, invoice_id).await?;

    let delivery_note_input = CreateDeliveryNoteInput {
        client_id: invoice.client_id,
        quote_id: invoice.quote_id,
        invoice_id: Some(invoice_id.to_string()),
        issue_date: chrono::Utc::now().date_naive(),
        delivery_date: None,
        delivery_address: None,
        notes: invoice.notes,
        lines: invoice.lines.into_iter().map(|l| CreateDeliveryNoteLineInput {
            product_id: l.product_id,
            description: l.description,
            quantity: l.quantity,
            unit: Some("unité".to_string()),
        }).collect(),
    };

    create_delivery_note(pool, delivery_note_input).await
}

// Convert Delivery Note to Invoice
pub async fn convert_delivery_note_to_invoice(pool: &PgPool, delivery_note_id: &str) -> Result<Invoice, sqlx::Error> {
    let delivery_note = get_delivery_note_by_id(pool, delivery_note_id).await?;
    let settings = get_company_settings(pool).await?;

    // Get product details for pricing
    let mut invoice_lines = Vec::new();
    for line in delivery_note.lines {
        let (unit_price_ht, vat_rate) = if let Some(product_id) = &line.product_id {
            if let Ok(product) = get_product_by_id(pool, product_id).await {
                (product.unit_price_ht, product.vat_rate)
            } else {
                (0.0, settings.default_vat_rate)
            }
        } else {
            (0.0, settings.default_vat_rate)
        };

        invoice_lines.push(CreateInvoiceLineInput {
            product_id: line.product_id,
            description: line.description,
            description_html: None,
            quantity: line.quantity,
            unit_price_ht,
            vat_rate,
            group_name: None,
            is_subtotal_line: None,
        });
    }

    let due_date = chrono::Utc::now().date_naive() + chrono::Duration::days(settings.default_payment_terms as i64);

    let invoice_input = CreateInvoiceInput {
        client_id: delivery_note.client_id,
        quote_id: delivery_note.quote_id,
        issue_date: chrono::Utc::now().date_naive(),
        due_date,
        notes: delivery_note.notes,
        notes_html: None,
        shipping_cost_ht: None,
        shipping_vat_rate: None,
        down_payment_percent: None,
        down_payment_amount: None,
        lines: invoice_lines,
    };

    // Don't decrease stock - already decreased when delivery note was created
    create_invoice_internal(pool, invoice_input, false).await
}

// Create Invoice from Multiple Delivery Notes
pub async fn create_invoice_from_delivery_notes(pool: &PgPool, delivery_note_ids: Vec<String>) -> Result<Invoice, sqlx::Error> {
    if delivery_note_ids.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    // Get the first delivery note to determine client
    let first_dn = get_delivery_note_by_id(pool, &delivery_note_ids[0]).await?;
    let client_id = first_dn.client_id.clone();
    let settings = get_company_settings(pool).await?;

    let mut all_lines = Vec::new();

    for dn_id in &delivery_note_ids {
        let dn = get_delivery_note_by_id(pool, dn_id).await?;

        // Verify all delivery notes are for the same client
        if dn.client_id != client_id {
            return Err(sqlx::Error::RowNotFound);
        }

        for line in dn.lines {
            let (unit_price_ht, vat_rate) = if let Some(product_id) = &line.product_id {
                if let Ok(product) = get_product_by_id(pool, product_id).await {
                    (product.unit_price_ht, product.vat_rate)
                } else {
                    (0.0, settings.default_vat_rate)
                }
            } else {
                (0.0, settings.default_vat_rate)
            };

            all_lines.push(CreateInvoiceLineInput {
                product_id: line.product_id,
                description: line.description,
                description_html: None,
                quantity: line.quantity,
                unit_price_ht,
                vat_rate,
                group_name: Some(format!("BL {}", dn.delivery_note_number)),
                is_subtotal_line: None,
            });
        }
    }

    let due_date = chrono::Utc::now().date_naive() + chrono::Duration::days(settings.default_payment_terms as i64);

    let invoice_input = CreateInvoiceInput {
        client_id,
        quote_id: None,
        issue_date: chrono::Utc::now().date_naive(),
        due_date,
        notes: None,
        notes_html: None,
        shipping_cost_ht: None,
        shipping_vat_rate: None,
        down_payment_percent: None,
        down_payment_amount: None,
        lines: all_lines,
    };

    // Don't decrease stock - already decreased when delivery notes were created
    create_invoice_internal(pool, invoice_input, false).await
}

// Client Contact Repository
pub async fn get_all_client_contacts(pool: &PgPool) -> Result<Vec<ClientContact>, sqlx::Error> {
    sqlx::query_as::<_, ClientContact>("SELECT * FROM client_contacts ORDER BY is_primary DESC, name")
        .fetch_all(pool)
        .await
}

pub async fn get_client_contacts_by_client_id(pool: &PgPool, client_id: &str) -> Result<Vec<ClientContact>, sqlx::Error> {
    sqlx::query_as::<_, ClientContact>("SELECT * FROM client_contacts WHERE client_id = $1 ORDER BY is_primary DESC, name")
        .bind(client_id)
        .fetch_all(pool)
        .await
}

pub async fn get_client_contact_by_id(pool: &PgPool, id: &str) -> Result<ClientContact, sqlx::Error> {
    sqlx::query_as::<_, ClientContact>("SELECT * FROM client_contacts WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_client_contact(pool: &PgPool, input: CreateClientContactInput) -> Result<ClientContact, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // If this is marked as primary, unset any existing primary contacts for this client
    if input.is_primary {
        sqlx::query("UPDATE client_contacts SET is_primary = false WHERE client_id = $1")
            .bind(&input.client_id)
            .execute(pool)
            .await?;
    }

    sqlx::query(
        r#"
        INSERT INTO client_contacts (id, client_id, name, role, email, phone, is_primary, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(&id)
    .bind(&input.client_id)
    .bind(&input.name)
    .bind(&input.role)
    .bind(&input.email)
    .bind(&input.phone)
    .bind(input.is_primary)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_client_contact_by_id(pool, &id).await
}

pub async fn update_client_contact(pool: &PgPool, input: UpdateClientContactInput) -> Result<ClientContact, sqlx::Error> {
    let now = Utc::now();

    // Get the contact first to know the client_id
    let contact = get_client_contact_by_id(pool, &input.id).await?;

    // If this is marked as primary, unset any existing primary contacts for this client
    if input.is_primary {
        sqlx::query("UPDATE client_contacts SET is_primary = false WHERE client_id = $1 AND id != $2")
            .bind(&contact.client_id)
            .bind(&input.id)
            .execute(pool)
            .await?;
    }

    sqlx::query(
        r#"
        UPDATE client_contacts SET name = $1, role = $2, email = $3, phone = $4, is_primary = $5, updated_at = $6
        WHERE id = $7
        "#,
    )
    .bind(&input.name)
    .bind(&input.role)
    .bind(&input.email)
    .bind(&input.phone)
    .bind(input.is_primary)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    get_client_contact_by_id(pool, &input.id).await
}

pub async fn delete_client_contact(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM client_contacts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Search all contacts (phonebook)
pub async fn search_contacts(pool: &PgPool, query: &str) -> Result<Vec<ClientContact>, sqlx::Error> {
    let search_pattern = format!("%{}%", query);
    sqlx::query_as::<_, ClientContact>(
        r#"
        SELECT * FROM client_contacts
        WHERE name LIKE $1 OR email LIKE $2 OR phone LIKE $3 OR role LIKE $4
        ORDER BY name
        "#
    )
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .fetch_all(pool)
    .await
}

// Reminder Repository
pub async fn get_all_reminders(pool: &PgPool) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ReminderRow>("SELECT * FROM reminders ORDER BY scheduled_date")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

pub async fn get_pending_reminders(pool: &PgPool) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ReminderRow>(
        "SELECT * FROM reminders WHERE sent_at IS NULL AND scheduled_date <= CURRENT_DATE ORDER BY scheduled_date"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

pub async fn get_reminders_by_document(pool: &PgPool, document_type: &str, document_id: &str) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ReminderRow>(
        "SELECT * FROM reminders WHERE document_type = $1 AND document_id = $2 ORDER BY scheduled_date"
    )
    .bind(document_type)
    .bind(document_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

pub async fn get_reminder_by_id(pool: &PgPool, id: &str) -> Result<Reminder, sqlx::Error> {
    let row = sqlx::query_as::<_, ReminderRow>("SELECT * FROM reminders WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(Reminder::from(row))
}

pub async fn create_reminder(pool: &PgPool, input: CreateReminderInput) -> Result<Reminder, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO reminders (id, reminder_type, document_type, document_id, scheduled_date, message, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&id)
    .bind(input.reminder_type.to_string())
    .bind(input.document_type.to_string())
    .bind(&input.document_id)
    .bind(&input.scheduled_date)
    .bind(&input.message)
    .bind(&now)
    .execute(pool)
    .await?;

    get_reminder_by_id(pool, &id).await
}

pub async fn mark_reminder_sent(pool: &PgPool, id: &str) -> Result<Reminder, sqlx::Error> {
    let now = Utc::now();
    sqlx::query("UPDATE reminders SET sent_at = $1 WHERE id = $2")
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

    get_reminder_by_id(pool, id).await
}

pub async fn delete_reminder(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM reminders WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Auto-create reminders for overdue invoices
pub async fn create_payment_due_reminders(pool: &PgPool) -> Result<Vec<Reminder>, sqlx::Error> {
    // Get all issued invoices that are past due date and don't have a pending reminder
    let overdue_invoices = sqlx::query_as::<_, InvoiceRow>(
        r#"
        SELECT i.* FROM invoices i
        WHERE i.status = 'ISSUED'
        AND i.due_date < CURRENT_DATE
        AND NOT EXISTS (
            SELECT 1 FROM reminders r
            WHERE r.document_type = 'INVOICE'
            AND r.document_id = i.id
            AND r.reminder_type = 'PAYMENT_DUE'
            AND r.sent_at IS NULL
        )
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut created_reminders = Vec::new();
    for invoice in overdue_invoices {
        let reminder = create_reminder(pool, CreateReminderInput {
            reminder_type: ReminderType::PaymentDue,
            document_type: DocumentType::Invoice,
            document_id: invoice.id,
            scheduled_date: chrono::Utc::now().date_naive(),
            message: Some("Paiement en retard".to_string()),
        }).await?;
        created_reminders.push(reminder);
    }
    Ok(created_reminders)
}

// Auto-create reminders for expiring quotes
pub async fn create_quote_expiring_reminders(pool: &PgPool) -> Result<Vec<Reminder>, sqlx::Error> {
    // Get all quotes expiring in 7 days that don't have a pending reminder
    let expiring_quotes = sqlx::query_as::<_, QuoteRow>(
        r#"
        SELECT q.* FROM quotes q
        WHERE q.status IN ('DRAFT', 'SENT')
        AND q.validity_date BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '7 days'
        AND NOT EXISTS (
            SELECT 1 FROM reminders r
            WHERE r.document_type = 'QUOTE'
            AND r.document_id = q.id
            AND r.reminder_type = 'QUOTE_EXPIRING'
            AND r.sent_at IS NULL
        )
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut created_reminders = Vec::new();
    for quote in expiring_quotes {
        let reminder = create_reminder(pool, CreateReminderInput {
            reminder_type: ReminderType::QuoteExpiring,
            document_type: DocumentType::Quote,
            document_id: quote.id,
            scheduled_date: chrono::Utc::now().date_naive(),
            message: Some("Devis expire bientôt".to_string()),
        }).await?;
        created_reminders.push(reminder);
    }
    Ok(created_reminders)
}

// Report Functions
pub async fn get_revenue_by_month(pool: &PgPool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<Vec<RevenueByPeriod>, sqlx::Error> {
    let start = start_date.map(|d| d.to_string()).unwrap_or_else(|| "2000-01-01".to_string());
    let end = end_date.map(|d| d.to_string()).unwrap_or_else(|| "2100-12-31".to_string());

    let rows = sqlx::query_as::<_, (String, f64, f64, i64)>(
        r#"
        SELECT
            to_char(issue_date, 'YYYY-MM') as period,
            COALESCE(SUM(total_ht), 0) as revenue_ht,
            COALESCE(SUM(total_ttc), 0) as revenue_ttc,
            COUNT(*) as invoice_count
        FROM invoices
        WHERE status IN ('ISSUED', 'PAID')
        AND issue_date >= $1 AND issue_date <= $2
        GROUP BY to_char(issue_date, 'YYYY-MM')
        ORDER BY period DESC
        "#
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(period, revenue_ht, revenue_ttc, invoice_count)| {
        RevenueByPeriod { period, revenue_ht, revenue_ttc, invoice_count }
    }).collect())
}

pub async fn get_revenue_by_client(pool: &PgPool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<Vec<RevenueByClient>, sqlx::Error> {
    let start = start_date.map(|d| d.to_string()).unwrap_or_else(|| "2000-01-01".to_string());
    let end = end_date.map(|d| d.to_string()).unwrap_or_else(|| "2100-12-31".to_string());

    let rows = sqlx::query_as::<_, (String, String, f64, f64, i64)>(
        r#"
        SELECT
            i.client_id,
            c.name as client_name,
            COALESCE(SUM(i.total_ht), 0) as revenue_ht,
            COALESCE(SUM(i.total_ttc), 0) as revenue_ttc,
            COUNT(*) as invoice_count
        FROM invoices i
        JOIN clients c ON i.client_id = c.id
        WHERE i.status IN ('ISSUED', 'PAID')
        AND i.issue_date >= $1 AND i.issue_date <= $2
        GROUP BY i.client_id, c.name
        ORDER BY revenue_ttc DESC
        "#
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(client_id, client_name, revenue_ht, revenue_ttc, invoice_count)| {
        RevenueByClient { client_id, client_name, revenue_ht, revenue_ttc, invoice_count }
    }).collect())
}

pub async fn get_product_sales(pool: &PgPool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<Vec<ProductSales>, sqlx::Error> {
    let start = start_date.map(|d| d.to_string()).unwrap_or_else(|| "2000-01-01".to_string());
    let end = end_date.map(|d| d.to_string()).unwrap_or_else(|| "2100-12-31".to_string());

    let rows = sqlx::query_as::<_, (String, String, f64, f64, f64)>(
        r#"
        SELECT
            COALESCE(il.product_id, 'custom') as product_id,
            COALESCE(p.designation, il.description) as product_name,
            COALESCE(SUM(il.quantity), 0) as quantity_sold,
            COALESCE(SUM(il.total_ht), 0) as revenue_ht,
            COALESCE(SUM(il.total_ttc), 0) as revenue_ttc
        FROM invoice_lines il
        JOIN invoices i ON il.invoice_id = i.id
        LEFT JOIN products p ON il.product_id = p.id
        WHERE i.status IN ('ISSUED', 'PAID')
        AND i.issue_date >= $1 AND i.issue_date <= $2
        GROUP BY il.product_id, COALESCE(p.designation, il.description)
        ORDER BY revenue_ttc DESC
        "#
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(product_id, product_name, quantity_sold, revenue_ht, revenue_ttc)| {
        ProductSales { product_id, product_name, quantity_sold, revenue_ht, revenue_ttc }
    }).collect())
}

pub async fn get_outstanding_payments(pool: &PgPool) -> Result<Vec<OutstandingPayment>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, f64)>(
        r#"
        SELECT
            i.id,
            i.invoice_number,
            c.name as client_name,
            i.issue_date,
            i.due_date,
            i.total_ttc
        FROM invoices i
        JOIN clients c ON i.client_id = c.id
        WHERE i.status = 'ISSUED'
        ORDER BY i.due_date ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    let today = chrono::Utc::now().date_naive();

    Ok(rows.into_iter().map(|(invoice_id, invoice_number, client_name, issue_date, due_date, total_ttc)| {
        let issue = chrono::NaiveDate::parse_from_str(&issue_date, "%Y-%m-%d").unwrap_or(today);
        let due = chrono::NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").unwrap_or(today);
        let days_overdue = (today - due).num_days().max(0);
        OutstandingPayment {
            invoice_id,
            invoice_number,
            client_name,
            issue_date: issue,
            due_date: due,
            total_ttc,
            days_overdue
        }
    }).collect())
}

pub async fn get_quote_conversion_stats(pool: &PgPool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<QuoteConversionStats, sqlx::Error> {
    let start = start_date.map(|d| d.to_string()).unwrap_or_else(|| "2000-01-01".to_string());
    let end = end_date.map(|d| d.to_string()).unwrap_or_else(|| "2100-12-31".to_string());

    let row = sqlx::query_as::<_, (i64, i64, f64, f64)>(
        r#"
        SELECT
            COUNT(*) as total_quotes,
            SUM(CASE WHEN status = 'ACCEPTED' THEN 1 ELSE 0 END) as converted_quotes,
            COALESCE(SUM(total_ttc), 0) as total_quoted_amount,
            COALESCE(SUM(CASE WHEN status = 'ACCEPTED' THEN total_ttc ELSE 0 END), 0) as converted_amount
        FROM quotes
        WHERE issue_date >= $1 AND issue_date <= $2
        "#
    )
    .bind(&start)
    .bind(&end)
    .fetch_one(pool)
    .await?;

    let (total_quotes, converted_quotes, total_quoted_amount, converted_amount) = row;
    let conversion_rate = if total_quotes > 0 { (converted_quotes as f64 / total_quotes as f64) * 100.0 } else { 0.0 };

    Ok(QuoteConversionStats {
        total_quotes,
        converted_quotes,
        conversion_rate,
        total_quoted_amount,
        converted_amount,
    })
}

// Alerts and Reminders
pub async fn get_alerts_summary(pool: &PgPool) -> Result<AlertsSummary, sqlx::Error> {
    use crate::models::Alert;

    let today = chrono::Utc::now().date_naive();
    let today_str = today.to_string();
    let soon_threshold = (today + chrono::Duration::days(7)).to_string();

    // Get overdue invoices (past due date, status = ISSUED)
    let overdue_rows: Vec<(String, String, String, String, f64)> = sqlx::query_as(
        r#"
        SELECT i.id, i.invoice_number, COALESCE(c.name, 'Unknown'), i.due_date, i.total_ttc
        FROM invoices i
        LEFT JOIN clients c ON i.client_id = c.id
        WHERE i.status = 'ISSUED' AND i.due_date < $1
        ORDER BY i.due_date ASC
        "#
    )
    .bind(&today_str)
    .fetch_all(pool)
    .await?;

    let overdue_invoices: Vec<Alert> = overdue_rows.into_iter().map(|(id, number, client, due_date, amount)| {
        let due = chrono::NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").unwrap_or(today);
        let days = (today - due).num_days() as i32;
        Alert {
            id: format!("overdue-{}", id),
            alert_type: "OVERDUE_INVOICE".to_string(),
            title: format!("Facture {} en retard", number),
            message: format!("{} jour(s) de retard - {}", days, client),
            document_type: "invoice".to_string(),
            document_id: id,
            document_number: number,
            client_name: client,
            amount: Some(amount),
            date: due_date,
            days,
            severity: if days > 30 { "danger".to_string() } else { "warning".to_string() },
        }
    }).collect();

    // Get invoices due soon (within 7 days)
    let due_soon_rows: Vec<(String, String, String, String, f64)> = sqlx::query_as(
        r#"
        SELECT i.id, i.invoice_number, COALESCE(c.name, 'Unknown'), i.due_date, i.total_ttc
        FROM invoices i
        LEFT JOIN clients c ON i.client_id = c.id
        WHERE i.status = 'ISSUED' AND i.due_date >= $1 AND i.due_date <= $2
        ORDER BY i.due_date ASC
        "#
    )
    .bind(&today_str)
    .bind(&soon_threshold)
    .fetch_all(pool)
    .await?;

    let due_soon_invoices: Vec<Alert> = due_soon_rows.into_iter().map(|(id, number, client, due_date, amount)| {
        let due = chrono::NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").unwrap_or(today);
        let days = (due - today).num_days() as i32;
        Alert {
            id: format!("due-soon-{}", id),
            alert_type: "DUE_SOON".to_string(),
            title: format!("Facture {} bientôt échue", number),
            message: format!("Échéance dans {} jour(s) - {}", days, client),
            document_type: "invoice".to_string(),
            document_id: id,
            document_number: number,
            client_name: client,
            amount: Some(amount),
            date: due_date,
            days: -days, // Negative for "days until"
            severity: "info".to_string(),
        }
    }).collect();

    // Get expiring quotes (within 7 days, status SENT or DRAFT)
    let expiring_rows: Vec<(String, String, String, String, f64)> = sqlx::query_as(
        r#"
        SELECT q.id, q.quote_number, COALESCE(c.name, 'Unknown'), q.validity_date, q.total_ttc
        FROM quotes q
        LEFT JOIN clients c ON q.client_id = c.id
        WHERE q.status IN ('DRAFT', 'SENT') AND q.validity_date >= $1 AND q.validity_date <= $2
        ORDER BY q.validity_date ASC
        "#
    )
    .bind(&today_str)
    .bind(&soon_threshold)
    .fetch_all(pool)
    .await?;

    let expiring_quotes: Vec<Alert> = expiring_rows.into_iter().map(|(id, number, client, validity_date, amount)| {
        let validity = chrono::NaiveDate::parse_from_str(&validity_date, "%Y-%m-%d").unwrap_or(today);
        let days = (validity - today).num_days() as i32;
        Alert {
            id: format!("expiring-{}", id),
            alert_type: "EXPIRING_QUOTE".to_string(),
            title: format!("Devis {} expire bientôt", number),
            message: format!("Expire dans {} jour(s) - {}", days, client),
            document_type: "quote".to_string(),
            document_id: id,
            document_number: number,
            client_name: client,
            amount: Some(amount),
            date: validity_date,
            days: -days,
            severity: if days <= 3 { "warning".to_string() } else { "info".to_string() },
        }
    }).collect();

    // Get expired quotes (past validity, still in SENT status)
    let expired_rows: Vec<(String, String, String, String, f64)> = sqlx::query_as(
        r#"
        SELECT q.id, q.quote_number, COALESCE(c.name, 'Unknown'), q.validity_date, q.total_ttc
        FROM quotes q
        LEFT JOIN clients c ON q.client_id = c.id
        WHERE q.status = 'SENT' AND q.validity_date < $1
        ORDER BY q.validity_date DESC
        "#
    )
    .bind(&today_str)
    .fetch_all(pool)
    .await?;

    let expired_quotes: Vec<Alert> = expired_rows.into_iter().map(|(id, number, client, validity_date, amount)| {
        let validity = chrono::NaiveDate::parse_from_str(&validity_date, "%Y-%m-%d").unwrap_or(today);
        let days = (today - validity).num_days() as i32;
        Alert {
            id: format!("expired-{}", id),
            alert_type: "EXPIRED_QUOTE".to_string(),
            title: format!("Devis {} expiré", number),
            message: format!("Expiré depuis {} jour(s) - {}", days, client),
            document_type: "quote".to_string(),
            document_id: id,
            document_number: number,
            client_name: client,
            amount: Some(amount),
            date: validity_date,
            days,
            severity: "danger".to_string(),
        }
    }).collect();

    let total_overdue_amount: f64 = overdue_invoices.iter().filter_map(|a| a.amount).sum();
    let total_count = (overdue_invoices.len() + due_soon_invoices.len() + expiring_quotes.len() + expired_quotes.len()) as i32;

    Ok(AlertsSummary {
        overdue_invoices,
        due_soon_invoices,
        expiring_quotes,
        expired_quotes,
        total_overdue_amount,
        total_count,
    })
}

// Mark quote as expired
pub async fn mark_quote_expired(pool: &PgPool, quote_id: &str) -> Result<Quote, sqlx::Error> {
    sqlx::query("UPDATE quotes SET status = 'EXPIRED', updated_at = $1 WHERE id = $2")
        .bind(chrono::Utc::now())
        .bind(quote_id)
        .execute(pool)
        .await?;

    get_quote_by_id(pool, quote_id).await
}

// Expense Repository
pub async fn get_all_expenses(pool: &PgPool) -> Result<Vec<Expense>, sqlx::Error> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses ORDER BY date DESC")
        .fetch_all(pool)
        .await
}

pub async fn get_expense_by_id(pool: &PgPool, id: &str) -> Result<Expense, sqlx::Error> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_expense(pool: &PgPool, input: CreateExpenseInput) -> Result<Expense, sqlx::Error> {
    let expense = Expense::new(input);
    sqlx::query(
        r#"
        INSERT INTO expenses (id, name, amount, date, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&expense.id)
    .bind(&expense.name)
    .bind(expense.amount)
    .bind(&expense.date)
    .bind(&expense.notes)
    .bind(&expense.created_at)
    .bind(&expense.updated_at)
    .execute(pool)
    .await?;

    Ok(expense)
}

pub async fn update_expense(pool: &PgPool, input: UpdateExpenseInput) -> Result<Expense, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE expenses SET name = $1, amount = $2, date = $3, notes = $4, updated_at = $5
        WHERE id = $6
        "#,
    )
    .bind(&input.name)
    .bind(input.amount)
    .bind(&input.date)
    .bind(&input.notes)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    get_expense_by_id(pool, &input.id).await
}

pub async fn delete_expense(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM expenses WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_expenses(pool: &PgPool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM expenses WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn restore_expense(pool: &PgPool, expense: Expense) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO expenses (id, name, amount, date, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&expense.id)
    .bind(&expense.name)
    .bind(expense.amount)
    .bind(&expense.date)
    .bind(&expense.notes)
    .bind(&expense.created_at)
    .bind(&expense.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

// Supplier Repository
pub async fn get_all_suppliers(pool: &PgPool) -> Result<Vec<Supplier>, sqlx::Error> {
    sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers ORDER BY name")
        .fetch_all(pool)
        .await
}

pub async fn get_supplier_by_id(pool: &PgPool, id: &str) -> Result<Supplier, sqlx::Error> {
    sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_supplier(pool: &PgPool, input: CreateSupplierInput) -> Result<Supplier, sqlx::Error> {
    let supplier = Supplier::new(input);
    sqlx::query(
        r#"
        INSERT INTO suppliers (id, name, email, phone, address, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&supplier.id)
    .bind(&supplier.name)
    .bind(&supplier.email)
    .bind(&supplier.phone)
    .bind(&supplier.address)
    .bind(&supplier.notes)
    .bind(&supplier.created_at)
    .bind(&supplier.updated_at)
    .execute(pool)
    .await?;

    Ok(supplier)
}

pub async fn update_supplier(pool: &PgPool, input: UpdateSupplierInput) -> Result<Supplier, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE suppliers SET name = $1, email = $2, phone = $3, address = $4, notes = $5, updated_at = $6
        WHERE id = $7
        "#,
    )
    .bind(&input.name)
    .bind(&input.email)
    .bind(&input.phone)
    .bind(&input.address)
    .bind(&input.notes)
    .bind(&now)
    .bind(&input.id)
    .execute(pool)
    .await?;

    get_supplier_by_id(pool, &input.id).await
}

pub async fn delete_supplier(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM suppliers WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_suppliers(pool: &PgPool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM suppliers WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn restore_supplier(pool: &PgPool, supplier: Supplier) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO suppliers (id, name, email, phone, address, notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&supplier.id)
    .bind(&supplier.name)
    .bind(&supplier.email)
    .bind(&supplier.phone)
    .bind(&supplier.address)
    .bind(&supplier.notes)
    .bind(&supplier.created_at)
    .bind(&supplier.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

// Product-Supplier Link Repository
pub async fn get_all_product_supplier_summaries(pool: &PgPool) -> Result<Vec<ProductSupplierSummary>, sqlx::Error> {
    sqlx::query_as::<_, ProductSupplierSummary>(
        r#"
        SELECT ps.product_id, ps.supplier_id, s.name as supplier_name
        FROM product_suppliers ps
        JOIN suppliers s ON ps.supplier_id = s.id
        ORDER BY s.name
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_suppliers_for_product(pool: &PgPool, product_id: &str) -> Result<Vec<SupplierWithPrice>, sqlx::Error> {
    sqlx::query_as::<_, SupplierWithPrice>(
        r#"
        SELECT s.id, s.name, s.email, s.phone, ps.purchase_price_ht, ps.id as link_id
        FROM product_suppliers ps
        JOIN suppliers s ON ps.supplier_id = s.id
        WHERE ps.product_id = $1
        ORDER BY s.name
        "#,
    )
    .bind(product_id)
    .fetch_all(pool)
    .await
}

pub async fn get_products_for_supplier(pool: &PgPool, supplier_id: &str) -> Result<Vec<ProductWithPrice>, sqlx::Error> {
    sqlx::query_as::<_, ProductWithPrice>(
        r#"
        SELECT p.id, p.designation, p.reference, p.unit_price_ht, ps.purchase_price_ht, ps.id as link_id
        FROM product_suppliers ps
        JOIN products p ON ps.product_id = p.id
        WHERE ps.supplier_id = $1
        ORDER BY p.designation
        "#,
    )
    .bind(supplier_id)
    .fetch_all(pool)
    .await
}

pub async fn add_product_supplier(pool: &PgPool, input: CreateProductSupplierInput) -> Result<ProductSupplier, sqlx::Error> {
    let link = ProductSupplier::new(input);
    sqlx::query(
        r#"
        INSERT INTO product_suppliers (id, product_id, supplier_id, purchase_price_ht, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&link.id)
    .bind(&link.product_id)
    .bind(&link.supplier_id)
    .bind(link.purchase_price_ht)
    .bind(&link.created_at)
    .execute(pool)
    .await?;

    // Recalculate product purchase price (set to cheapest supplier)
    recalculate_product_purchase_price(pool, &link.product_id).await?;

    Ok(link)
}

pub async fn remove_product_supplier(pool: &PgPool, link_id: &str) -> Result<(), sqlx::Error> {
    // Get the product_id before deleting
    let link = sqlx::query_as::<_, ProductSupplier>("SELECT * FROM product_suppliers WHERE id = $1")
        .bind(link_id)
        .fetch_one(pool)
        .await?;

    sqlx::query("DELETE FROM product_suppliers WHERE id = $1")
        .bind(link_id)
        .execute(pool)
        .await?;

    // Recalculate product purchase price
    recalculate_product_purchase_price(pool, &link.product_id).await?;

    Ok(())
}

pub async fn update_product_supplier_price(pool: &PgPool, link_id: &str, purchase_price_ht: f64) -> Result<(), sqlx::Error> {
    let link = sqlx::query_as::<_, ProductSupplier>("SELECT * FROM product_suppliers WHERE id = $1")
        .bind(link_id)
        .fetch_one(pool)
        .await?;

    sqlx::query("UPDATE product_suppliers SET purchase_price_ht = $1 WHERE id = $2")
        .bind(purchase_price_ht)
        .bind(link_id)
        .execute(pool)
        .await?;

    // Recalculate product purchase price
    recalculate_product_purchase_price(pool, &link.product_id).await?;

    Ok(())
}

async fn recalculate_product_purchase_price(pool: &PgPool, product_id: &str) -> Result<(), sqlx::Error> {
    let min_price: Option<f64> = sqlx::query_scalar(
        "SELECT MIN(purchase_price_ht) FROM product_suppliers WHERE product_id = $1"
    )
    .bind(product_id)
    .fetch_one(pool)
    .await?;

    if let Some(price) = min_price {
        sqlx::query("UPDATE products SET purchase_price_ht = $1, updated_at = $2 WHERE id = $3")
            .bind(price)
            .bind(Utc::now())
            .bind(product_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn restore_product_supplier(pool: &PgPool, ps: ProductSupplier) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO product_suppliers (id, product_id, supplier_id, purchase_price_ht, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&ps.id)
    .bind(&ps.product_id)
    .bind(&ps.supplier_id)
    .bind(ps.purchase_price_ht)
    .bind(&ps.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

// User Repository
pub async fn check_any_users_exist(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    Ok(row.0 > 0)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at")
        .fetch_all(pool)
        .await
}

pub async fn get_user_by_id(pool: &PgPool, id: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
}

pub async fn create_user(pool: &PgPool, id: &str, username: &str, display_name: &str, password_hash: &str, role: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        INSERT INTO users (id, username, display_name, password_hash, role, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, true, $6, $7)
        "#,
    )
    .bind(id)
    .bind(username)
    .bind(display_name)
    .bind(password_hash)
    .bind(role)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_user(pool: &PgPool, id: &str, username: &str, display_name: &str, role: &str, is_active: bool) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE users SET username = $1, display_name = $2, role = $3, is_active = $4, updated_at = $5
        WHERE id = $6
        "#,
    )
    .bind(username)
    .bind(display_name)
    .bind(role)
    .bind(is_active)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_user_password(pool: &PgPool, id: &str, password_hash: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query("UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3")
        .bind(password_hash)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_user(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn count_admin_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE role = 'admin'")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn get_user_permissions(pool: &PgPool, user_id: &str) -> Result<Vec<UserPermission>, sqlx::Error> {
    sqlx::query_as::<_, UserPermission>("SELECT * FROM user_permissions WHERE user_id = $1 AND granted = true")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn set_user_permissions(pool: &PgPool, user_id: &str, permissions: &[String]) -> Result<(), sqlx::Error> {
    // Delete existing permissions
    sqlx::query("DELETE FROM user_permissions WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    // Insert new permissions
    for perm in permissions {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO user_permissions (id, user_id, permission_key, granted)
            VALUES ($1, $2, $3, true)
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(perm)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn build_user_info(pool: &PgPool, user: &User) -> Result<UserInfo, sqlx::Error> {
    let permissions = if user.role == "admin" {
        ALL_PERMISSIONS.iter().map(|s| s.to_string()).collect()
    } else {
        let perms = get_user_permissions(pool, &user.id).await?;
        perms.into_iter().map(|p| p.permission_key).collect()
    };

    Ok(UserInfo {
        id: user.id.clone(),
        username: user.username.clone(),
        display_name: user.display_name.clone(),
        role: user.role.clone(),
        is_active: user.is_active,
        permissions,
        created_at: user.created_at.to_rfc3339(),
        updated_at: user.updated_at.to_rfc3339(),
    })
}

// User backup/restore functions
pub async fn get_all_users_for_backup(pool: &PgPool) -> Result<Vec<UserBackup>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at")
        .fetch_all(pool)
        .await?;
    Ok(users.into_iter().map(|u| UserBackup {
        id: u.id,
        username: u.username,
        display_name: u.display_name,
        password_hash: u.password_hash,
        role: u.role,
        is_active: u.is_active,
        created_at: u.created_at.to_rfc3339(),
        updated_at: u.updated_at.to_rfc3339(),
    }).collect())
}

pub async fn get_all_user_permissions_for_backup(pool: &PgPool) -> Result<Vec<UserPermissionBackup>, sqlx::Error> {
    let perms = sqlx::query_as::<_, UserPermission>("SELECT * FROM user_permissions")
        .fetch_all(pool)
        .await?;
    Ok(perms.into_iter().map(|p| UserPermissionBackup {
        id: p.id,
        user_id: p.user_id,
        permission_key: p.permission_key,
        granted: p.granted,
    }).collect())
}

pub async fn restore_user(pool: &PgPool, user: UserBackup) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO users (id, username, display_name, password_hash, role, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.display_name)
    .bind(&user.password_hash)
    .bind(&user.role)
    .bind(user.is_active)
    .bind(&user.created_at)
    .bind(&user.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn restore_user_permission(pool: &PgPool, perm: UserPermissionBackup) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO user_permissions (id, user_id, permission_key, granted)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&perm.id)
    .bind(&perm.user_id)
    .bind(&perm.permission_key)
    .bind(perm.granted)
    .execute(pool)
    .await?;
    Ok(())
}
