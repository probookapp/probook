use chrono::Utc;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
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
pub async fn get_all_clients(pool: &SqlitePool) -> Result<Vec<Client>, sqlx::Error> {
    sqlx::query_as::<_, Client>("SELECT * FROM clients ORDER BY name")
        .fetch_all(pool)
        .await
}

pub async fn get_client_by_id(pool: &SqlitePool, id: &str) -> Result<Client, sqlx::Error> {
    sqlx::query_as::<_, Client>("SELECT * FROM clients WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_client(pool: &SqlitePool, input: CreateClientInput) -> Result<Client, sqlx::Error> {
    let client = Client::new(input);
    sqlx::query(
        r#"
        INSERT INTO clients (id, name, email, phone, address, city, postal_code, country, siret, vat_number, notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_client(pool: &SqlitePool, input: UpdateClientInput) -> Result<Client, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE clients SET name = ?, email = ?, phone = ?, address = ?, city = ?, postal_code = ?, country = ?, siret = ?, vat_number = ?, notes = ?, updated_at = ?
        WHERE id = ?
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

pub async fn delete_client(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM clients WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_clients(pool: &SqlitePool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM clients WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

// Product Repository
pub async fn get_all_products(pool: &SqlitePool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY designation")
        .fetch_all(pool)
        .await
}

pub async fn get_product_by_id(pool: &SqlitePool, id: &str) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_product(pool: &SqlitePool, input: CreateProductInput) -> Result<Product, sqlx::Error> {
    let product = Product::new(input);
    sqlx::query(
        r#"
        INSERT INTO products (id, designation, description, unit_price_ht, vat_rate, unit, reference, is_service, category_id, quantity, purchase_price_ht, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_product(pool: &SqlitePool, input: UpdateProductInput) -> Result<Product, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE products SET designation = ?, description = ?, unit_price_ht = ?, vat_rate = ?, unit = ?, reference = ?, is_service = ?, category_id = ?, quantity = ?, purchase_price_ht = ?, updated_at = ?
        WHERE id = ?
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

pub async fn delete_product(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_products(pool: &SqlitePool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

// Decrease product stock by quantity (only for non-service products)
pub async fn decrease_product_stock(pool: &SqlitePool, product_id: &str, quantity: f64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products SET quantity = MAX(0, COALESCE(quantity, 0) - ?), updated_at = ?
        WHERE id = ? AND is_service = 0
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
pub async fn get_all_quotes(pool: &SqlitePool) -> Result<Vec<Quote>, sqlx::Error> {
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

pub async fn get_quote_by_id(pool: &SqlitePool, id: &str) -> Result<Quote, sqlx::Error> {
    let row = sqlx::query_as::<_, QuoteRow>("SELECT * FROM quotes WHERE id = ?")
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

async fn get_quote_lines(pool: &SqlitePool, quote_id: &str) -> Result<Vec<QuoteLine>, sqlx::Error> {
    sqlx::query_as::<_, QuoteLine>("SELECT * FROM quote_lines WHERE quote_id = ? ORDER BY position")
        .bind(quote_id)
        .fetch_all(pool)
        .await
}

pub async fn create_quote(pool: &SqlitePool, input: CreateQuoteInput) -> Result<Quote, sqlx::Error> {
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
        VALUES (?, ?, ?, 'DRAFT', ?, ?, ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_quote(pool: &SqlitePool, input: UpdateQuoteInput, logo_snapshot: Option<String>) -> Result<Quote, sqlx::Error> {
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
    sqlx::query("DELETE FROM quote_lines WHERE quote_id = ?")
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
        UPDATE quotes SET client_id = ?, status = ?, issue_date = ?, validity_date = ?, total_ht = ?, total_vat = ?, total_ttc = ?, notes = ?, logo_snapshot = ?, updated_at = ?
        WHERE id = ?
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn delete_quote(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM quotes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_quotes(pool: &SqlitePool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM quotes WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

// Invoice Repository (similar structure to quotes)
pub async fn get_all_invoices(pool: &SqlitePool) -> Result<Vec<Invoice>, sqlx::Error> {
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
            is_down_payment_invoice: row.is_down_payment_invoice.map(|v| v != 0).unwrap_or(false),
            parent_quote_id: row.parent_quote_id,
            lines,
            payments,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }
    Ok(invoices)
}

pub async fn get_invoice_by_id(pool: &SqlitePool, id: &str) -> Result<Invoice, sqlx::Error> {
    let row = sqlx::query_as::<_, InvoiceRow>("SELECT * FROM invoices WHERE id = ?")
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
        is_down_payment_invoice: row.is_down_payment_invoice.map(|v| v != 0).unwrap_or(false),
        parent_quote_id: row.parent_quote_id,
        lines,
        payments,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

async fn get_invoice_lines(pool: &SqlitePool, invoice_id: &str) -> Result<Vec<InvoiceLine>, sqlx::Error> {
    sqlx::query_as::<_, InvoiceLine>("SELECT * FROM invoice_lines WHERE invoice_id = ? ORDER BY position")
        .bind(invoice_id)
        .fetch_all(pool)
        .await
}

pub async fn create_invoice(pool: &SqlitePool, input: CreateInvoiceInput) -> Result<Invoice, sqlx::Error> {
    create_invoice_internal(pool, input, true).await
}

async fn create_invoice_internal(pool: &SqlitePool, input: CreateInvoiceInput, decrease_stock: bool) -> Result<Invoice, sqlx::Error> {
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
        VALUES (?, ?, ?, ?, 'DRAFT', ?, ?, ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_invoice(pool: &SqlitePool, input: UpdateInvoiceInput) -> Result<Invoice, sqlx::Error> {
    let now = Utc::now();

    // Delete existing lines
    sqlx::query("DELETE FROM invoice_lines WHERE invoice_id = ?")
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
        UPDATE invoices SET client_id = ?, status = ?, issue_date = ?, due_date = ?, total_ht = ?, total_vat = ?, total_ttc = ?, notes = ?, updated_at = ?
        WHERE id = ?
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn delete_invoice(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM invoices WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_invoices(pool: &SqlitePool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        let row: (String,) = sqlx::query_as("SELECT status FROM invoices WHERE id = ?")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;
        if row.0 != "DRAFT" {
            return Err(sqlx::Error::Protocol(
                format!("Cannot delete non-DRAFT invoice {}", id),
            ));
        }
        sqlx::query("DELETE FROM invoices WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn mark_invoice_paid(pool: &SqlitePool, id: &str) -> Result<Invoice, sqlx::Error> {
    let now = Utc::now();

    // First get the invoice to compute hash if not already set
    let invoice = get_invoice_by_id(pool, id).await?;

    // If no integrity hash, compute and set it
    if invoice.integrity_hash.is_none() {
        let hash = compute_invoice_hash(&invoice);
        sqlx::query("UPDATE invoices SET status = 'PAID', integrity_hash = ?, updated_at = ? WHERE id = ?")
            .bind(&hash)
            .bind(&now)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE invoices SET status = 'PAID', updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(id)
            .execute(pool)
            .await?;
    }

    get_invoice_by_id(pool, id).await
}

pub async fn issue_invoice(pool: &SqlitePool, id: &str, logo_snapshot: Option<String>) -> Result<Invoice, sqlx::Error> {
    let now = Utc::now();

    // Get the invoice and compute integrity hash
    let invoice = get_invoice_by_id(pool, id).await?;
    let hash = compute_invoice_hash(&invoice);

    sqlx::query("UPDATE invoices SET status = 'ISSUED', integrity_hash = ?, logo_snapshot = ?, updated_at = ? WHERE id = ?")
        .bind(&hash)
        .bind(&logo_snapshot)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

    get_invoice_by_id(pool, id).await
}

pub async fn verify_invoice_integrity(pool: &SqlitePool, id: &str) -> Result<bool, sqlx::Error> {
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
pub async fn get_all_payments(pool: &SqlitePool) -> Result<Vec<Payment>, sqlx::Error> {
    sqlx::query_as::<_, Payment>("SELECT * FROM payments ORDER BY payment_date DESC")
        .fetch_all(pool)
        .await
}

pub async fn get_payments_by_invoice(pool: &SqlitePool, invoice_id: &str) -> Result<Vec<Payment>, sqlx::Error> {
    sqlx::query_as::<_, Payment>("SELECT * FROM payments WHERE invoice_id = ? ORDER BY payment_date DESC")
        .bind(invoice_id)
        .fetch_all(pool)
        .await
}

pub async fn create_payment(pool: &SqlitePool, input: CreatePaymentInput) -> Result<Payment, sqlx::Error> {
    let invoice_id = input.invoice_id.clone();
    let payment = Payment::new(input);
    sqlx::query(
        r#"
        INSERT INTO payments (id, invoice_id, amount, payment_date, payment_method, reference, notes, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn delete_payment(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM payments WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Company Settings Repository
pub async fn get_company_settings(pool: &SqlitePool) -> Result<CompanySettings, sqlx::Error> {
    sqlx::query_as::<_, CompanySettings>("SELECT * FROM company_settings WHERE id = 'default'")
        .fetch_one(pool)
        .await
}

pub async fn update_company_settings(pool: &SqlitePool, input: UpdateCompanySettingsInput) -> Result<CompanySettings, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE company_settings SET
            company_name = ?, address = ?, city = ?, postal_code = ?, country = ?,
            phone = ?, email = ?, website = ?, siret = ?, vat_number = ?,
            default_vat_rate = ?, default_payment_terms = ?, invoice_prefix = ?, quote_prefix = ?,
            legal_mentions = ?, bank_details = ?, updated_at = ?
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
    .bind(&now)
    .execute(pool)
    .await?;

    get_company_settings(pool).await
}

pub async fn update_last_backup_date(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE company_settings SET last_backup_date = ? WHERE id = 'default'")
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_app_settings(
    pool: &SqlitePool,
    app_language: &str,
    app_theme: &str,
    auto_update_enabled: bool,
) -> Result<CompanySettings, sqlx::Error> {
    let now = Utc::now();
    let auto_update_int: i32 = if auto_update_enabled { 1 } else { 0 };
    sqlx::query(
        r#"
        UPDATE company_settings SET
            app_language = ?,
            app_theme = ?,
            auto_update_enabled = ?,
            updated_at = ?
        WHERE id = 'default'
        "#,
    )
    .bind(app_language)
    .bind(app_theme)
    .bind(auto_update_int)
    .bind(&now)
    .execute(pool)
    .await?;

    get_company_settings(pool).await
}

// Dashboard Stats
pub async fn get_dashboard_stats(pool: &SqlitePool) -> Result<DashboardStats, sqlx::Error> {
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
        "SELECT COALESCE(SUM(total_ttc), 0.0) FROM invoices WHERE status = 'PAID' AND strftime('%Y-%m', issue_date) = strftime('%Y-%m', 'now')"
    )
    .fetch_one(pool)
    .await?;

    let revenue_this_year: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total_ttc), 0.0) FROM invoices WHERE status = 'PAID' AND strftime('%Y', issue_date) = strftime('%Y', 'now')"
    )
    .fetch_one(pool)
    .await?;

    let pending_payments: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(total_ttc), 0.0) FROM invoices WHERE status = 'ISSUED'"
    )
    .fetch_one(pool)
    .await?;

    let total_expenses: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount), 0.0) FROM expenses WHERE strftime('%Y', date) = strftime('%Y', 'now')"
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
pub async fn duplicate_quote(pool: &SqlitePool, quote_id: &str) -> Result<Quote, sqlx::Error> {
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
        VALUES (?, ?, ?, 'DRAFT', ?, ?, ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
pub async fn duplicate_invoice(pool: &SqlitePool, invoice_id: &str) -> Result<Invoice, sqlx::Error> {
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
        VALUES (?, ?, ?, NULL, 'DRAFT', ?, ?, ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
pub async fn convert_quote_to_invoice(pool: &SqlitePool, quote_id: &str) -> Result<Invoice, sqlx::Error> {
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
            is_subtotal_line: l.is_subtotal_line.map(|v| v != 0),
        }).collect(),
    };

    create_invoice(pool, invoice_input).await
}

// Logo Management
pub async fn update_logo_path(pool: &SqlitePool, logo_path: &str) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    let path = if logo_path.is_empty() {
        None
    } else {
        Some(logo_path.to_string())
    };
    sqlx::query("UPDATE company_settings SET logo_path = ?, updated_at = ? WHERE id = 'default'")
        .bind(path)
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(())
}

// Backup Restore Functions
pub async fn clear_all_data(pool: &SqlitePool) -> Result<(), sqlx::Error> {
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
    Ok(())
}

pub async fn restore_client(pool: &SqlitePool, client: Client) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO clients (id, name, email, phone, address, city, postal_code, country, siret, vat_number, notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn restore_product(pool: &SqlitePool, product: Product) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO products (id, designation, description, unit_price_ht, vat_rate, unit, reference, is_service, category_id, quantity, purchase_price_ht, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn restore_quote(pool: &SqlitePool, quote: Quote) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO quotes (id, quote_number, client_id, status, issue_date, validity_date, total_ht, total_vat, total_ttc, notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn restore_invoice(pool: &SqlitePool, invoice: Invoice) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO invoices (id, invoice_number, client_id, quote_id, status, issue_date, due_date, total_ht, total_vat, total_ttc, notes, integrity_hash, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn restore_payment(pool: &SqlitePool, payment: Payment) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO payments (id, invoice_id, amount, payment_date, payment_method, reference, notes, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn restore_settings(pool: &SqlitePool, settings: CompanySettings) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE company_settings SET
            company_name = ?, address = ?, city = ?, postal_code = ?, country = ?,
            phone = ?, email = ?, website = ?, siret = ?, vat_number = ?, logo_path = ?,
            default_vat_rate = ?, default_payment_terms = ?, invoice_prefix = ?, quote_prefix = ?,
            next_invoice_number = ?, next_quote_number = ?, legal_mentions = ?, bank_details = ?, updated_at = ?
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
    .bind(&settings.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

// Product Category Repository
pub async fn get_all_product_categories(pool: &SqlitePool) -> Result<Vec<ProductCategory>, sqlx::Error> {
    sqlx::query_as::<_, ProductCategory>("SELECT * FROM product_categories ORDER BY name")
        .fetch_all(pool)
        .await
}

pub async fn get_product_category_by_id(pool: &SqlitePool, id: &str) -> Result<ProductCategory, sqlx::Error> {
    sqlx::query_as::<_, ProductCategory>("SELECT * FROM product_categories WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_product_category(pool: &SqlitePool, input: CreateProductCategoryInput) -> Result<ProductCategory, sqlx::Error> {
    let category = ProductCategory::new(input);
    sqlx::query(
        r#"
        INSERT INTO product_categories (id, name, description, parent_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
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

pub async fn update_product_category(pool: &SqlitePool, input: UpdateProductCategoryInput) -> Result<ProductCategory, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE product_categories SET name = ?, description = ?, parent_id = ?, updated_at = ?
        WHERE id = ?
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

pub async fn delete_product_category(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    // First, unset category_id for all products in this category
    sqlx::query("UPDATE products SET category_id = NULL WHERE category_id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    // Then delete the category
    sqlx::query("DELETE FROM product_categories WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Update product photo path
pub async fn update_product_photo(pool: &SqlitePool, product_id: &str, photo_path: Option<&str>) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query("UPDATE products SET photo_path = ?, updated_at = ? WHERE id = ?")
        .bind(photo_path)
        .bind(&now)
        .bind(product_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Delivery Note Repository
pub async fn get_all_delivery_notes(pool: &SqlitePool) -> Result<Vec<DeliveryNote>, sqlx::Error> {
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

pub async fn get_delivery_note_by_id(pool: &SqlitePool, id: &str) -> Result<DeliveryNote, sqlx::Error> {
    let row = sqlx::query_as::<_, DeliveryNoteRow>("SELECT * FROM delivery_notes WHERE id = ?")
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

async fn get_delivery_note_lines(pool: &SqlitePool, delivery_note_id: &str) -> Result<Vec<DeliveryNoteLine>, sqlx::Error> {
    sqlx::query_as::<_, DeliveryNoteLine>("SELECT * FROM delivery_note_lines WHERE delivery_note_id = ? ORDER BY position")
        .bind(delivery_note_id)
        .fetch_all(pool)
        .await
}

pub async fn create_delivery_note(pool: &SqlitePool, input: CreateDeliveryNoteInput) -> Result<DeliveryNote, sqlx::Error> {
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
        VALUES (?, ?, ?, ?, ?, 'DRAFT', ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_delivery_note(pool: &SqlitePool, input: UpdateDeliveryNoteInput) -> Result<DeliveryNote, sqlx::Error> {
    let now = Utc::now();

    // Delete existing lines
    sqlx::query("DELETE FROM delivery_note_lines WHERE delivery_note_id = ?")
        .bind(&input.id)
        .execute(pool)
        .await?;

    let lines: Vec<DeliveryNoteLine> = input.lines.iter().enumerate()
        .map(|(i, l)| DeliveryNoteLine::new(&input.id, l.clone(), i as i32))
        .collect();

    sqlx::query(
        r#"
        UPDATE delivery_notes SET client_id = ?, quote_id = ?, invoice_id = ?, status = ?, issue_date = ?, delivery_date = ?, delivery_address = ?, notes = ?, updated_at = ?
        WHERE id = ?
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn delete_delivery_note(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM delivery_notes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_delivery_notes(pool: &SqlitePool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM delivery_notes WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn duplicate_delivery_note(pool: &SqlitePool, delivery_note_id: &str) -> Result<DeliveryNote, sqlx::Error> {
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
        VALUES (?, ?, ?, ?, ?, 'DRAFT', ?, ?, ?, ?, ?, ?)
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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
pub async fn convert_quote_to_delivery_note(pool: &SqlitePool, quote_id: &str) -> Result<DeliveryNote, sqlx::Error> {
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
pub async fn convert_invoice_to_delivery_note(pool: &SqlitePool, invoice_id: &str) -> Result<DeliveryNote, sqlx::Error> {
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
pub async fn convert_delivery_note_to_invoice(pool: &SqlitePool, delivery_note_id: &str) -> Result<Invoice, sqlx::Error> {
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
pub async fn create_invoice_from_delivery_notes(pool: &SqlitePool, delivery_note_ids: Vec<String>) -> Result<Invoice, sqlx::Error> {
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
pub async fn get_all_client_contacts(pool: &SqlitePool) -> Result<Vec<ClientContact>, sqlx::Error> {
    sqlx::query_as::<_, ClientContact>("SELECT * FROM client_contacts ORDER BY is_primary DESC, name")
        .fetch_all(pool)
        .await
}

pub async fn get_client_contacts_by_client_id(pool: &SqlitePool, client_id: &str) -> Result<Vec<ClientContact>, sqlx::Error> {
    sqlx::query_as::<_, ClientContact>("SELECT * FROM client_contacts WHERE client_id = ? ORDER BY is_primary DESC, name")
        .bind(client_id)
        .fetch_all(pool)
        .await
}

pub async fn get_client_contact_by_id(pool: &SqlitePool, id: &str) -> Result<ClientContact, sqlx::Error> {
    sqlx::query_as::<_, ClientContact>("SELECT * FROM client_contacts WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_client_contact(pool: &SqlitePool, input: CreateClientContactInput) -> Result<ClientContact, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // If this is marked as primary, unset any existing primary contacts for this client
    if input.is_primary {
        sqlx::query("UPDATE client_contacts SET is_primary = 0 WHERE client_id = ?")
            .bind(&input.client_id)
            .execute(pool)
            .await?;
    }

    sqlx::query(
        r#"
        INSERT INTO client_contacts (id, client_id, name, role, email, phone, is_primary, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_client_contact(pool: &SqlitePool, input: UpdateClientContactInput) -> Result<ClientContact, sqlx::Error> {
    let now = Utc::now();

    // Get the contact first to know the client_id
    let contact = get_client_contact_by_id(pool, &input.id).await?;

    // If this is marked as primary, unset any existing primary contacts for this client
    if input.is_primary {
        sqlx::query("UPDATE client_contacts SET is_primary = 0 WHERE client_id = ? AND id != ?")
            .bind(&contact.client_id)
            .bind(&input.id)
            .execute(pool)
            .await?;
    }

    sqlx::query(
        r#"
        UPDATE client_contacts SET name = ?, role = ?, email = ?, phone = ?, is_primary = ?, updated_at = ?
        WHERE id = ?
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

pub async fn delete_client_contact(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM client_contacts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Search all contacts (phonebook)
pub async fn search_contacts(pool: &SqlitePool, query: &str) -> Result<Vec<ClientContact>, sqlx::Error> {
    let search_pattern = format!("%{}%", query);
    sqlx::query_as::<_, ClientContact>(
        r#"
        SELECT * FROM client_contacts
        WHERE name LIKE ? OR email LIKE ? OR phone LIKE ? OR role LIKE ?
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
pub async fn get_all_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ReminderRow>("SELECT * FROM reminders ORDER BY scheduled_date")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

pub async fn get_pending_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ReminderRow>(
        "SELECT * FROM reminders WHERE sent_at IS NULL AND scheduled_date <= date('now') ORDER BY scheduled_date"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

pub async fn get_reminders_by_document(pool: &SqlitePool, document_type: &str, document_id: &str) -> Result<Vec<Reminder>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ReminderRow>(
        "SELECT * FROM reminders WHERE document_type = ? AND document_id = ? ORDER BY scheduled_date"
    )
    .bind(document_type)
    .bind(document_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Reminder::from).collect())
}

pub async fn get_reminder_by_id(pool: &SqlitePool, id: &str) -> Result<Reminder, sqlx::Error> {
    let row = sqlx::query_as::<_, ReminderRow>("SELECT * FROM reminders WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(Reminder::from(row))
}

pub async fn create_reminder(pool: &SqlitePool, input: CreateReminderInput) -> Result<Reminder, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO reminders (id, reminder_type, document_type, document_id, scheduled_date, message, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
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

pub async fn mark_reminder_sent(pool: &SqlitePool, id: &str) -> Result<Reminder, sqlx::Error> {
    let now = Utc::now();
    sqlx::query("UPDATE reminders SET sent_at = ? WHERE id = ?")
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

    get_reminder_by_id(pool, id).await
}

pub async fn delete_reminder(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM reminders WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Auto-create reminders for overdue invoices
pub async fn create_payment_due_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, sqlx::Error> {
    // Get all issued invoices that are past due date and don't have a pending reminder
    let overdue_invoices = sqlx::query_as::<_, InvoiceRow>(
        r#"
        SELECT i.* FROM invoices i
        WHERE i.status = 'ISSUED'
        AND i.due_date < date('now')
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
pub async fn create_quote_expiring_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, sqlx::Error> {
    // Get all quotes expiring in 7 days that don't have a pending reminder
    let expiring_quotes = sqlx::query_as::<_, QuoteRow>(
        r#"
        SELECT q.* FROM quotes q
        WHERE q.status IN ('DRAFT', 'SENT')
        AND q.validity_date BETWEEN date('now') AND date('now', '+7 days')
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
pub async fn get_revenue_by_month(pool: &SqlitePool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<Vec<RevenueByPeriod>, sqlx::Error> {
    let start = start_date.map(|d| d.to_string()).unwrap_or_else(|| "2000-01-01".to_string());
    let end = end_date.map(|d| d.to_string()).unwrap_or_else(|| "2100-12-31".to_string());

    let rows = sqlx::query_as::<_, (String, f64, f64, i64)>(
        r#"
        SELECT
            strftime('%Y-%m', issue_date) as period,
            COALESCE(SUM(total_ht), 0) as revenue_ht,
            COALESCE(SUM(total_ttc), 0) as revenue_ttc,
            COUNT(*) as invoice_count
        FROM invoices
        WHERE status IN ('ISSUED', 'PAID')
        AND issue_date >= ? AND issue_date <= ?
        GROUP BY strftime('%Y-%m', issue_date)
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

pub async fn get_revenue_by_client(pool: &SqlitePool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<Vec<RevenueByClient>, sqlx::Error> {
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
        AND i.issue_date >= ? AND i.issue_date <= ?
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

pub async fn get_product_sales(pool: &SqlitePool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<Vec<ProductSales>, sqlx::Error> {
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
        AND i.issue_date >= ? AND i.issue_date <= ?
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

pub async fn get_outstanding_payments(pool: &SqlitePool) -> Result<Vec<OutstandingPayment>, sqlx::Error> {
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

pub async fn get_quote_conversion_stats(pool: &SqlitePool, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) -> Result<QuoteConversionStats, sqlx::Error> {
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
        WHERE issue_date >= ? AND issue_date <= ?
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
pub async fn get_alerts_summary(pool: &SqlitePool) -> Result<AlertsSummary, sqlx::Error> {
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
        WHERE i.status = 'ISSUED' AND i.due_date < ?
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
        WHERE i.status = 'ISSUED' AND i.due_date >= ? AND i.due_date <= ?
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
        WHERE q.status IN ('DRAFT', 'SENT') AND q.validity_date >= ? AND q.validity_date <= ?
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
        WHERE q.status = 'SENT' AND q.validity_date < ?
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
pub async fn mark_quote_expired(pool: &SqlitePool, quote_id: &str) -> Result<Quote, sqlx::Error> {
    sqlx::query("UPDATE quotes SET status = 'EXPIRED', updated_at = ? WHERE id = ?")
        .bind(chrono::Utc::now())
        .bind(quote_id)
        .execute(pool)
        .await?;

    get_quote_by_id(pool, quote_id).await
}

// Expense Repository
pub async fn get_all_expenses(pool: &SqlitePool) -> Result<Vec<Expense>, sqlx::Error> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses ORDER BY date DESC")
        .fetch_all(pool)
        .await
}

pub async fn get_expense_by_id(pool: &SqlitePool, id: &str) -> Result<Expense, sqlx::Error> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_expense(pool: &SqlitePool, input: CreateExpenseInput) -> Result<Expense, sqlx::Error> {
    let expense = Expense::new(input);
    sqlx::query(
        r#"
        INSERT INTO expenses (id, name, amount, date, notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_expense(pool: &SqlitePool, input: UpdateExpenseInput) -> Result<Expense, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE expenses SET name = ?, amount = ?, date = ?, notes = ?, updated_at = ?
        WHERE id = ?
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

pub async fn delete_expense(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM expenses WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_expenses(pool: &SqlitePool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM expenses WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn restore_expense(pool: &SqlitePool, expense: Expense) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO expenses (id, name, amount, date, notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
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
pub async fn get_all_suppliers(pool: &SqlitePool) -> Result<Vec<Supplier>, sqlx::Error> {
    sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers ORDER BY name")
        .fetch_all(pool)
        .await
}

pub async fn get_supplier_by_id(pool: &SqlitePool, id: &str) -> Result<Supplier, sqlx::Error> {
    sqlx::query_as::<_, Supplier>("SELECT * FROM suppliers WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_supplier(pool: &SqlitePool, input: CreateSupplierInput) -> Result<Supplier, sqlx::Error> {
    let supplier = Supplier::new(input);
    sqlx::query(
        r#"
        INSERT INTO suppliers (id, name, email, phone, address, notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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

pub async fn update_supplier(pool: &SqlitePool, input: UpdateSupplierInput) -> Result<Supplier, sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        r#"
        UPDATE suppliers SET name = ?, email = ?, phone = ?, address = ?, notes = ?, updated_at = ?
        WHERE id = ?
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

pub async fn delete_supplier(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM suppliers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn batch_delete_suppliers(pool: &SqlitePool, ids: Vec<String>) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut count: u64 = 0;
    for id in &ids {
        sqlx::query("DELETE FROM suppliers WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    Ok(count)
}

pub async fn restore_supplier(pool: &SqlitePool, supplier: Supplier) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO suppliers (id, name, email, phone, address, notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
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
pub async fn get_all_product_supplier_summaries(pool: &SqlitePool) -> Result<Vec<ProductSupplierSummary>, sqlx::Error> {
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

pub async fn get_suppliers_for_product(pool: &SqlitePool, product_id: &str) -> Result<Vec<SupplierWithPrice>, sqlx::Error> {
    sqlx::query_as::<_, SupplierWithPrice>(
        r#"
        SELECT s.id, s.name, s.email, s.phone, ps.purchase_price_ht, ps.id as link_id
        FROM product_suppliers ps
        JOIN suppliers s ON ps.supplier_id = s.id
        WHERE ps.product_id = ?
        ORDER BY s.name
        "#,
    )
    .bind(product_id)
    .fetch_all(pool)
    .await
}

pub async fn get_products_for_supplier(pool: &SqlitePool, supplier_id: &str) -> Result<Vec<ProductWithPrice>, sqlx::Error> {
    sqlx::query_as::<_, ProductWithPrice>(
        r#"
        SELECT p.id, p.designation, p.reference, p.unit_price_ht, ps.purchase_price_ht, ps.id as link_id
        FROM product_suppliers ps
        JOIN products p ON ps.product_id = p.id
        WHERE ps.supplier_id = ?
        ORDER BY p.designation
        "#,
    )
    .bind(supplier_id)
    .fetch_all(pool)
    .await
}

pub async fn add_product_supplier(pool: &SqlitePool, input: CreateProductSupplierInput) -> Result<ProductSupplier, sqlx::Error> {
    let link = ProductSupplier::new(input);
    sqlx::query(
        r#"
        INSERT INTO product_suppliers (id, product_id, supplier_id, purchase_price_ht, created_at)
        VALUES (?, ?, ?, ?, ?)
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

pub async fn remove_product_supplier(pool: &SqlitePool, link_id: &str) -> Result<(), sqlx::Error> {
    // Get the product_id before deleting
    let link = sqlx::query_as::<_, ProductSupplier>("SELECT * FROM product_suppliers WHERE id = ?")
        .bind(link_id)
        .fetch_one(pool)
        .await?;

    sqlx::query("DELETE FROM product_suppliers WHERE id = ?")
        .bind(link_id)
        .execute(pool)
        .await?;

    // Recalculate product purchase price
    recalculate_product_purchase_price(pool, &link.product_id).await?;

    Ok(())
}

pub async fn update_product_supplier_price(pool: &SqlitePool, link_id: &str, purchase_price_ht: f64) -> Result<(), sqlx::Error> {
    let link = sqlx::query_as::<_, ProductSupplier>("SELECT * FROM product_suppliers WHERE id = ?")
        .bind(link_id)
        .fetch_one(pool)
        .await?;

    sqlx::query("UPDATE product_suppliers SET purchase_price_ht = ? WHERE id = ?")
        .bind(purchase_price_ht)
        .bind(link_id)
        .execute(pool)
        .await?;

    // Recalculate product purchase price
    recalculate_product_purchase_price(pool, &link.product_id).await?;

    Ok(())
}

async fn recalculate_product_purchase_price(pool: &SqlitePool, product_id: &str) -> Result<(), sqlx::Error> {
    let min_price: Option<f64> = sqlx::query_scalar(
        "SELECT MIN(purchase_price_ht) FROM product_suppliers WHERE product_id = ?"
    )
    .bind(product_id)
    .fetch_one(pool)
    .await?;

    if let Some(price) = min_price {
        sqlx::query("UPDATE products SET purchase_price_ht = ?, updated_at = ? WHERE id = ?")
            .bind(price)
            .bind(Utc::now())
            .bind(product_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn restore_product_supplier(pool: &SqlitePool, ps: ProductSupplier) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO product_suppliers (id, product_id, supplier_id, purchase_price_ht, created_at)
        VALUES (?, ?, ?, ?, ?)
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
