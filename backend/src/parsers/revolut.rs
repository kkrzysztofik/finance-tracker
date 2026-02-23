use super::{ParsedTransaction, common};
use chrono::NaiveDate;
use serde_json::json;

/// Expected header columns (after mojibake fix):
/// Rodzaj, Produkt, Data rozpoczęcia, Data zrealizowania, Opis, Kwota, Opłata, Waluta, State, Saldo
const COL_RODZAJ: usize = 0;
const COL_DATA_ROZPOCZECIA: usize = 2;
const COL_DATA_ZREALIZOWANIA: usize = 3;
const COL_OPIS: usize = 4;
const COL_KWOTA: usize = 5;
const COL_WALUTA: usize = 7;
const COL_STATE: usize = 8;
const COL_SALDO: usize = 9;

const MIN_COLUMNS: usize = 10;

/// Parse a Revolut CSV export.
///
/// The file may be double-encoded UTF-8 (mojibake). We fix the entire content
/// first, strip carriage returns, then parse as standard comma-delimited CSV.
pub fn parse(content: &str) -> Result<Vec<ParsedTransaction>, String> {
    // Step 1: Fix mojibake on entire content
    let fixed = common::fix_mojibake(content);

    // Step 2: Strip \r characters (Windows line endings)
    let cleaned = fixed.replace('\r', "");

    // Step 3: Parse CSV with comma delimiter
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::Fields)
        .from_reader(cleaned.as_bytes());

    let mut transactions = Vec::new();

    for (row_idx, result) in reader.records().enumerate() {
        let record = result.map_err(|e| format!("CSV parse error at row {}: {}", row_idx + 2, e))?;

        if record.len() < MIN_COLUMNS {
            tracing::warn!(
                "Revolut row {} has only {} columns, expected {}, skipping",
                row_idx + 2,
                record.len(),
                MIN_COLUMNS
            );
            continue;
        }

        // Skip rows where Data rozpoczęcia is empty
        let date_str = record.get(COL_DATA_ROZPOCZECIA).unwrap_or("").trim();
        if date_str.is_empty() {
            continue;
        }

        // Parse transaction date (first 10 chars of "YYYY-MM-DD HH:MM:SS")
        let transaction_date = parse_date_prefix(date_str)
            .map_err(|e| format!("Row {}: invalid transaction date '{}': {}", row_idx + 2, date_str, e))?;

        // Parse booking date (may be empty)
        let booking_date_str = record.get(COL_DATA_ZREALIZOWANIA).unwrap_or("").trim();
        let booking_date = if booking_date_str.is_empty() {
            None
        } else {
            Some(parse_date_prefix(booking_date_str)
                .map_err(|e| format!("Row {}: invalid booking date '{}': {}", row_idx + 2, booking_date_str, e))?)
        };

        let description = common::normalize_whitespace(record.get(COL_OPIS).unwrap_or("").trim());
        let rodzaj = common::normalize_whitespace(record.get(COL_RODZAJ).unwrap_or("").trim());
        let currency = record.get(COL_WALUTA).unwrap_or("").trim().to_string();
        let state_raw = record.get(COL_STATE).unwrap_or("").trim().to_string();
        let saldo_str = record.get(COL_SALDO).unwrap_or("").trim().to_string();

        // Parse amount
        let amount_str = record.get(COL_KWOTA).unwrap_or("").trim();
        let amount = common::parse_polish_decimal(amount_str)
            .map_err(|e| format!("Row {}: {}", row_idx + 2, e))?;

        // Map state: after mojibake fix these should be proper Polish
        let state = map_state(&state_raw);

        // Extract counterparty from description if present
        let counterparty = extract_counterparty(&description);

        // Build raw_data JSON for audit trail
        let raw_data = json!({
            "rodzaj": rodzaj,
            "produkt": record.get(1).unwrap_or("").trim(),
            "data_rozpoczecia": date_str,
            "data_zrealizowania": booking_date_str,
            "opis": &description,
            "kwota": amount_str,
            "oplata": record.get(6).unwrap_or("").trim(),
            "waluta": &currency,
            "state": &state_raw,
            "saldo": &saldo_str,
        });

        transactions.push(ParsedTransaction {
            account: "revolut".to_string(),
            transaction_date,
            booking_date,
            counterparty,
            description,
            amount,
            currency,
            bank_category: None,
            bank_reference: None,
            bank_type: if rodzaj.is_empty() { None } else { Some(rodzaj) },
            state,
            raw_data,
        });
    }

    if transactions.is_empty() {
        return Err("No valid transactions found in Revolut CSV".into());
    }

    tracing::info!("Parsed {} Revolut transactions", transactions.len());
    Ok(transactions)
}

/// Parse the date portion from "YYYY-MM-DD HH:MM:SS" (takes first 10 chars).
fn parse_date_prefix(s: &str) -> Result<NaiveDate, String> {
    let date_part = if s.len() >= 10 { &s[..10] } else { s };
    NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
        .map_err(|e| format!("date parse failed: {}", e))
}

/// Map Revolut state values to normalized English strings.
///
/// After mojibake fix, the Polish values should be:
/// - "ZAKOŃCZONO" → "completed"
/// - "COFNIĘTO"  → "reversed"
/// - anything else → "pending"
fn map_state(state: &str) -> String {
    let upper = state.trim().to_uppercase();
    if upper.contains("ZAKON") || upper.contains("ZAKOŃCZONO") {
        "completed".to_string()
    } else if upper.contains("COFNI") || upper.contains("COFNIĘTO") {
        "reversed".to_string()
    } else {
        "pending".to_string()
    }
}

/// Try to extract a counterparty name from the description.
///
/// Known patterns:
/// - "Przelew do: SOME NAME" → "SOME NAME"
/// - "Przelew od: SOME NAME" → "SOME NAME"
/// - "Zasilenie o *XXXX" → None (card top-up, no meaningful counterparty)
/// - Otherwise the description itself might be a merchant name (e.g. "Steam")
fn extract_counterparty(description: &str) -> Option<String> {
    // "Przelew do: NAME" or "Przelew od: NAME"
    if let Some(pos) = description.find("do:") {
        let name = description[pos + 3..].trim();
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    if let Some(pos) = description.find("od:") {
        let name = description[pos + 3..].trim();
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }

    // Skip top-up descriptions
    if description.starts_with("Zasilenie") {
        return None;
    }

    // If the description looks like a short merchant name (no prefix patterns),
    // treat it as a counterparty
    if !description.contains(':') && !description.is_empty() && description.len() < 100 {
        return Some(description.to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn sample_csv() -> &'static str {
        "Rodzaj,Produkt,Data rozpoczęcia,Data zrealizowania,Opis,Kwota,Opłata,Waluta,State,Saldo\r\n\
         Zasilenie,Bieżące,2019-06-27 10:35:51,2019-06-27 10:36:53,Zasilenie o *3821,20,0,PLN,ZAKOŃCZONO,20\r\n\
         Płatność kartą,Bieżące,2019-06-28 20:59:31,2019-07-02 20:48:15,Steam,\"-7,19\",0,PLN,ZAKOŃCZONO,\"12,81\"\r\n\
         Przelew,Bieżące,2019-07-09 14:55:40,2019-07-09 14:55:40,Przelew do: RAFAL ASMAR SOUDANI,\"-2,5\",0,PLN,ZAKOŃCZONO,\"77,31\"\r\n"
    }

    #[test]
    fn test_parse_revolut_basic() {
        let result = parse(sample_csv()).expect("should parse successfully");
        assert_eq!(result.len(), 3);

        // First row: top-up
        assert_eq!(result[0].account, "revolut");
        assert_eq!(result[0].transaction_date, NaiveDate::from_ymd_opt(2019, 6, 27).unwrap());
        assert_eq!(result[0].amount, Decimal::from_str("20").unwrap());
        assert_eq!(result[0].currency, "PLN");
        assert_eq!(result[0].state, "completed");
        assert_eq!(result[0].bank_type.as_deref(), Some("Zasilenie"));
        assert!(result[0].counterparty.is_none()); // top-up, no counterparty

        // Second row: card payment
        assert_eq!(result[1].transaction_date, NaiveDate::from_ymd_opt(2019, 6, 28).unwrap());
        assert_eq!(result[1].booking_date, Some(NaiveDate::from_ymd_opt(2019, 7, 2).unwrap()));
        assert_eq!(result[1].amount, Decimal::from_str("-7.19").unwrap());
        assert_eq!(result[1].description, "Steam");
        assert_eq!(result[1].counterparty.as_deref(), Some("Steam"));

        // Third row: transfer with counterparty
        assert_eq!(result[2].amount, Decimal::from_str("-2.5").unwrap());
        assert_eq!(result[2].counterparty.as_deref(), Some("RAFAL ASMAR SOUDANI"));
        assert_eq!(result[2].description, "Przelew do: RAFAL ASMAR SOUDANI");
    }

    #[test]
    fn test_parse_date_prefix() {
        let d = parse_date_prefix("2019-06-27 10:35:51").unwrap();
        assert_eq!(d, NaiveDate::from_ymd_opt(2019, 6, 27).unwrap());
    }

    #[test]
    fn test_map_state() {
        assert_eq!(map_state("ZAKOŃCZONO"), "completed");
        assert_eq!(map_state("COFNIĘTO"), "reversed");
        assert_eq!(map_state(""), "pending");
        assert_eq!(map_state("SOMETHING_ELSE"), "pending");
    }

    #[test]
    fn test_extract_counterparty() {
        assert_eq!(
            extract_counterparty("Przelew do: JAN KOWALSKI"),
            Some("JAN KOWALSKI".to_string())
        );
        assert_eq!(
            extract_counterparty("Przelew od: ANNA NOWAK"),
            Some("ANNA NOWAK".to_string())
        );
        assert_eq!(extract_counterparty("Zasilenie o *3821"), None);
        assert_eq!(extract_counterparty("Steam"), Some("Steam".to_string()));
    }

    #[test]
    fn test_empty_date_skipped() {
        let csv = "Rodzaj,Produkt,Data rozpoczęcia,Data zrealizowania,Opis,Kwota,Opłata,Waluta,State,Saldo\r\n\
                   Zasilenie,Bieżące,,2019-06-27 10:36:53,Zasilenie,20,0,PLN,ZAKOŃCZONO,20\r\n";
        let result = parse(csv);
        assert!(result.is_err()); // no valid transactions
    }
}
