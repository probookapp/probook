use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub added: usize,
    pub updated: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

impl ImportResult {
    pub fn new() -> Self {
        Self {
            added: 0,
            updated: 0,
            skipped: 0,
            errors: Vec::new(),
        }
    }
}

pub type ParsedData = (Vec<String>, Vec<Vec<String>>);

/// Parse a CSV file into headers and rows
pub fn parse_csv(file_path: &str) -> Result<ParsedData, String> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(file_path)
        .map_err(|e| format!("Failed to open CSV file: {}", e))?;

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| format!("Failed to read CSV headers: {}", e))?
        .iter()
        .map(|h| h.trim().to_string())
        .collect();

    let mut rows = Vec::new();
    for (i, result) in reader.records().enumerate() {
        match result {
            Ok(record) => {
                let row: Vec<String> = record.iter().map(|f| f.trim().to_string()).collect();
                rows.push(row);
            }
            Err(e) => {
                return Err(format!("Error reading row {}: {}", i + 2, e));
            }
        }
    }

    Ok((headers, rows))
}

/// Parse an XLSX file into headers and rows
pub fn parse_xlsx(file_path: &str) -> Result<ParsedData, String> {
    use calamine::{open_workbook, Reader, Xlsx};

    let mut workbook: Xlsx<_> = open_workbook(file_path)
        .map_err(|e| format!("Failed to open Excel file: {}", e))?;

    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| "No sheets found in Excel file".to_string())?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| format!("Failed to read sheet: {}", e))?;

    let mut rows_iter = range.rows();

    // First row is headers
    let headers: Vec<String> = rows_iter
        .next()
        .ok_or_else(|| "Excel file is empty".to_string())?
        .iter()
        .map(|cell| cell.to_string().trim().to_string())
        .collect();

    let mut rows = Vec::new();
    for row in rows_iter {
        let row_data: Vec<String> = row.iter().map(|cell| cell.to_string().trim().to_string()).collect();
        // Skip completely empty rows
        if row_data.iter().all(|s| s.is_empty()) {
            continue;
        }
        rows.push(row_data);
    }

    Ok((headers, rows))
}

/// Parse a file based on its extension
pub fn parse_file(file_path: &str) -> Result<ParsedData, String> {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "csv" => parse_csv(file_path),
        "xlsx" | "xls" => parse_xlsx(file_path),
        _ => Err(format!("Unsupported file format: .{}. Use .csv or .xlsx", ext)),
    }
}

/// Validate that required columns exist in the headers
pub fn validate_columns(headers: &[String], required: &[&str], optional: &[&str]) -> Result<(), String> {
    let header_lower: Vec<String> = headers.iter().map(|h| h.to_lowercase()).collect();

    let mut missing = Vec::new();
    for col in required {
        if !header_lower.contains(&col.to_lowercase()) {
            missing.push(*col);
        }
    }

    if !missing.is_empty() {
        let all_valid: Vec<String> = required.iter().chain(optional.iter()).map(|s| s.to_string()).collect();
        return Err(format!(
            "Missing required columns: {}. Expected columns: {}",
            missing.join(", "),
            all_valid.join(", ")
        ));
    }

    Ok(())
}

/// Get a value from a row by column name (case-insensitive)
pub fn get_field(headers: &[String], row: &[String], field_name: &str) -> Option<String> {
    let field_lower = field_name.to_lowercase();
    headers
        .iter()
        .position(|h| h.to_lowercase() == field_lower)
        .and_then(|i| row.get(i))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
