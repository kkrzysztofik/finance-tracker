use super::{ParsedTransaction, common};
use chrono::NaiveDate;
use serde_json::json;

/// Alior CSV column indices (semicolon-delimited):
/// Data transakcji;Data księgowania;Nazwa nadawcy;Nazwa odbiorcy;Szczegóły transakcji;
/// Kwota operacji;Waluta operacji;Kwota w walucie rachunku;Waluta rachunku;
/// Numer rachunku nadawcy;Numer rachunku odbiorcy
const COL_DATA_TRANSAKCJI: usize = 0;
const COL_DATA_KSIEGOWANIA: usize = 1;
const COL_NAZWA_NADAWCY: usize = 2;
const COL_NAZWA_ODBIORCY: usize = 3;
const COL_SZCZEGOLY: usize = 4;
const COL_KWOTA_OPERACJI: usize = 5;
const COL_WALUTA_OPERACJI: usize = 6;
const COL_KWOTA_RACHUNKU: usize = 7;
const COL_WALUTA_RACHUNKU: usize = 8;
const COL_RACHUNEK_NADAWCY: usize = 9;
const COL_RACHUNEK_ODBIORCY: usize = 10;

const MIN_COLUMNS: usize = 9;

/// Parse an Alior Bank CSV export.
///
/// Format:
/// - Line 1: Metadata starting with "Kryteria transakcji:" — skipped
/// - Line 2: Headers (semicolon-delimited)
/// - Lines 3+: Data rows (semicolon-delimited)
/// - Dates: DD-MM-YYYY
/// - Amounts: Polish decimal format (-1180,00)
pub fn parse(content: &str) -> Result<Vec<ParsedTransaction>, String> {
    let cleaned = content.replace('\r', "");

    // Skip the metadata line ("Kryteria transakcji:...")
    let csv_content = if cleaned.starts_with("Kryteria transakcji") {
        match cleaned.find('\n') {
            Some(pos) => &cleaned[pos + 1..],
            None => return Err("Alior CSV contains only metadata line".into()),
        }
    } else {
        &cleaned
    };

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::Fields)
        .from_reader(csv_content.as_bytes());

    let mut transactions = Vec::new();

    for (row_idx, result) in reader.records().enumerate() {
        let record = result.map_err(|e| format!("CSV parse error at row {}: {}", row_idx + 2, e))?;

        if record.len() < MIN_COLUMNS {
            tracing::warn!(
                "Alior row {} has only {} columns, expected {}, skipping",
                row_idx + 2,
                record.len(),
                MIN_COLUMNS
            );
            continue;
        }

        let date_str = record.get(COL_DATA_TRANSAKCJI).unwrap_or("").trim();
        if date_str.is_empty() {
            continue;
        }

        // Parse transaction date (DD-MM-YYYY)
        let transaction_date = NaiveDate::parse_from_str(date_str, "%d-%m-%Y")
            .map_err(|e| format!("Row {}: invalid transaction date '{}': {}", row_idx + 2, date_str, e))?;

        // Parse booking date (DD-MM-YYYY, may be empty)
        let booking_date_str = record.get(COL_DATA_KSIEGOWANIA).unwrap_or("").trim();
        let booking_date = if booking_date_str.is_empty() {
            None
        } else {
            Some(NaiveDate::parse_from_str(booking_date_str, "%d-%m-%Y")
                .map_err(|e| format!("Row {}: invalid booking date '{}': {}", row_idx + 2, booking_date_str, e))?)
        };

        let nadawca = common::normalize_whitespace(record.get(COL_NAZWA_NADAWCY).unwrap_or("").trim());
        let odbiorca = common::normalize_whitespace(record.get(COL_NAZWA_ODBIORCY).unwrap_or("").trim());
        let details = common::normalize_whitespace(record.get(COL_SZCZEGOLY).unwrap_or("").trim());

        // Parse amount (use account-currency amount if available, otherwise operation amount)
        let amount_str = record.get(COL_KWOTA_RACHUNKU).unwrap_or("").trim();
        let amount_str = if amount_str.is_empty() {
            record.get(COL_KWOTA_OPERACJI).unwrap_or("").trim()
        } else {
            amount_str
        };
        let amount = common::parse_polish_decimal(amount_str)
            .map_err(|e| format!("Row {}: {}", row_idx + 2, e))?;

        // Currency: prefer account currency, fallback to operation currency
        let currency = {
            let waluta_rachunku = record.get(COL_WALUTA_RACHUNKU).unwrap_or("").trim();
            if waluta_rachunku.is_empty() {
                record.get(COL_WALUTA_OPERACJI).unwrap_or("PLN").trim().to_string()
            } else {
                waluta_rachunku.to_string()
            }
        };

        // Counterparty: if expense (negative), use recipient; if income, use sender
        let counterparty = if amount.is_sign_negative() {
            if !odbiorca.is_empty() {
                Some(odbiorca.clone())
            } else if !details.is_empty() {
                Some(details.clone())
            } else {
                None
            }
        } else if !nadawca.is_empty() {
            Some(nadawca.clone())
        } else if !details.is_empty() {
            Some(details.clone())
        } else {
            None
        };

        // Description from details
        let description = if details.is_empty() {
            if amount.is_sign_negative() {
                odbiorca.clone()
            } else {
                nadawca.clone()
            }
        } else {
            details.clone()
        };

        let raw_data = json!({
            "data_transakcji": date_str,
            "data_ksiegowania": booking_date_str,
            "nazwa_nadawcy": &nadawca,
            "nazwa_odbiorcy": &odbiorca,
            "szczegoly_transakcji": &details,
            "kwota_operacji": record.get(COL_KWOTA_OPERACJI).unwrap_or("").trim(),
            "waluta_operacji": record.get(COL_WALUTA_OPERACJI).unwrap_or("").trim(),
            "kwota_w_walucie_rachunku": record.get(COL_KWOTA_RACHUNKU).unwrap_or("").trim(),
            "waluta_rachunku": record.get(COL_WALUTA_RACHUNKU).unwrap_or("").trim(),
            "numer_rachunku_nadawcy": record.get(COL_RACHUNEK_NADAWCY).unwrap_or("").trim(),
            "numer_rachunku_odbiorcy": record.get(COL_RACHUNEK_ODBIORCY).unwrap_or("").trim(),
        });

        transactions.push(ParsedTransaction {
            account: "alior".to_string(),
            transaction_date,
            booking_date,
            counterparty,
            description,
            amount,
            currency,
            bank_category: None,
            bank_reference: None,
            bank_type: None,
            state: "completed".to_string(),
            raw_data,
        });
    }

    if transactions.is_empty() {
        return Err("No valid transactions found in Alior CSV".into());
    }

    tracing::info!("Parsed {} Alior transactions", transactions.len());
    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn sample_csv() -> &'static str {
        "Kryteria transakcji: od 2026-01-01 do 2026-02-23\n\
         Data transakcji;Data księgowania;Nazwa nadawcy;Nazwa odbiorcy;Szczegóły transakcji;Kwota operacji;Waluta operacji;Kwota w walucie rachunku;Waluta rachunku;Numer rachunku nadawcy;Numer rachunku odbiorcy\n\
         23-02-2026;23-02-2026;Jan Kowalski;;Przelew przychodzący;340,00;PLN;340,00;PLN;11222233334444555566667777;99887766554433221100998877\n\
         22-02-2026;22-02-2026;;Fitness Club;Za treningi;-1180,00;PLN;-1180,00;PLN;99887766554433221100998877;55443322110099887766554433\n\
         21-02-2026;21-02-2026;;;Opłata za kartę;-7,19;PLN;-7,19;PLN;99887766554433221100998877;\n"
    }

    #[test]
    fn test_parse_alior_basic() {
        let result = parse(sample_csv()).expect("should parse successfully");
        assert_eq!(result.len(), 3);

        // First row: incoming transfer
        assert_eq!(result[0].account, "alior");
        assert_eq!(result[0].transaction_date, NaiveDate::from_ymd_opt(2026, 2, 23).unwrap());
        assert_eq!(result[0].booking_date, Some(NaiveDate::from_ymd_opt(2026, 2, 23).unwrap()));
        assert_eq!(result[0].amount, Decimal::from_str("340.00").unwrap());
        assert_eq!(result[0].currency, "PLN");
        assert_eq!(result[0].state, "completed");
        assert_eq!(result[0].counterparty.as_deref(), Some("Jan Kowalski"));
        assert_eq!(result[0].description, "Przelew przychodzący");

        // Second row: expense
        assert_eq!(result[1].transaction_date, NaiveDate::from_ymd_opt(2026, 2, 22).unwrap());
        assert_eq!(result[1].amount, Decimal::from_str("-1180.00").unwrap());
        assert_eq!(result[1].counterparty.as_deref(), Some("Fitness Club"));
        assert_eq!(result[1].description, "Za treningi");

        // Third row: expense with no counterparty names — falls back to details
        assert_eq!(result[2].amount, Decimal::from_str("-7.19").unwrap());
        assert_eq!(result[2].counterparty.as_deref(), Some("Opłata za kartę"));
        assert_eq!(result[2].description, "Opłata za kartę");
    }

    #[test]
    fn test_parse_alior_no_metadata() {
        // CSV without the metadata line should also work
        let csv = "Data transakcji;Data księgowania;Nazwa nadawcy;Nazwa odbiorcy;Szczegóły transakcji;Kwota operacji;Waluta operacji;Kwota w walucie rachunku;Waluta rachunku;Numer rachunku nadawcy;Numer rachunku odbiorcy\n\
                   23-02-2026;23-02-2026;Jan Kowalski;;Przelew;340,00;PLN;340,00;PLN;1122;9988\n";
        let result = parse(csv).expect("should parse without metadata");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].counterparty.as_deref(), Some("Jan Kowalski"));
    }

    #[test]
    fn test_parse_alior_empty() {
        let csv = "Kryteria transakcji: brak\n\
                   Data transakcji;Data księgowania;Nazwa nadawcy;Nazwa odbiorcy;Szczegóły transakcji;Kwota operacji;Waluta operacji;Kwota w walucie rachunku;Waluta rachunku;Numer rachunku nadawcy;Numer rachunku odbiorcy\n";
        let result = parse(csv);
        assert!(result.is_err());
    }
}
