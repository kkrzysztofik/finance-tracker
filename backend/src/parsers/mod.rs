pub mod alior;
pub mod common;
pub mod pekao;
pub mod revolut;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A parsed transaction ready for insertion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTransaction {
    pub account: String,
    pub transaction_date: NaiveDate,
    pub booking_date: Option<NaiveDate>,
    pub counterparty: Option<String>,
    pub description: String,
    pub amount: Decimal,
    pub currency: String,
    pub bank_category: Option<String>,
    pub bank_reference: Option<String>,
    pub bank_type: Option<String>,
    pub state: String,
    pub raw_data: serde_json::Value,
}

/// Detect bank format from file content and parse accordingly
pub fn detect_and_parse(filename: &str, content: &str) -> Result<Vec<ParsedTransaction>, String> {
    // Try to detect based on filename patterns
    let lower = filename.to_lowercase();

    if lower.contains("historia_operacji") {
        tracing::info!("Detected Alior format from filename");
        return alior::parse(content);
    }

    if lower.contains("lista_operacji") {
        tracing::info!("Detected Pekao format from filename");
        return pekao::parse(content);
    }

    if lower.contains("account-statement") || lower.contains("revolut") {
        tracing::info!("Detected Revolut format from filename");
        return revolut::parse(content);
    }

    // Fallback: detect from content
    let first_lines: String = content.lines().take(3).collect::<Vec<_>>().join("\n");

    if first_lines.contains("Kryteria transakcji") {
        tracing::info!("Detected Alior format from content");
        return alior::parse(content);
    }

    if first_lines.contains("Data ksiegowania") || first_lines.contains("Data księgowania") || first_lines.contains("Kategoria") {
        tracing::info!("Detected Pekao format from content");
        return pekao::parse(content);
    }

    if first_lines.contains("Rodzaj") && first_lines.contains("Produkt") {
        tracing::info!("Detected Revolut format from content");
        return revolut::parse(content);
    }

    Err("Unable to detect CSV format. Supported: Alior (Historia_Operacji_*), Pekao (Lista_operacji_*), Revolut (account-statement_*)".into())
}
