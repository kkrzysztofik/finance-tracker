use super::{ParsedTransaction, common};
use chrono::NaiveDate;
use serde_json::json;

/// Pekao CSV column indices (semicolon-delimited):
/// Data księgowania;Data waluty;Nadawca / Odbiorca;Adres nadawcy / odbiorcy;
/// Rachunek Źródłowy;Rachunek docelowy;Tytuł;Kwota operacji;Waluta;
/// Numer referencyjny;Typ operacji;Kategoria;Mile transakcyjne
const COL_DATA_KSIEGOWANIA: usize = 0;
const COL_DATA_WALUTY: usize = 1;
const COL_NADAWCA_ODBIORCA: usize = 2;
const COL_ADRES: usize = 3;
const COL_RACHUNEK_ZRODLOWY: usize = 4;
const COL_RACHUNEK_DOCELOWY: usize = 5;
const COL_TYTUL: usize = 6;
const COL_KWOTA: usize = 7;
const COL_WALUTA: usize = 8;
const COL_NUMER_REFERENCYJNY: usize = 9;
const COL_TYP_OPERACJI: usize = 10;
const COL_KATEGORIA: usize = 11;
const COL_MILE: usize = 12;

const MIN_COLUMNS: usize = 9;

/// Strip a leading single-quote character from Pekao fields.
/// Pekao exports sometimes prefix account numbers with ' (e.g. '52470000...).
fn strip_leading_quote(s: &str) -> &str {
    s.strip_prefix('\'').unwrap_or(s)
}

/// Parse a Pekao SA CSV export.
///
/// Format:
/// - Line 1: Headers (semicolon-delimited)
/// - Lines 2+: Data rows (semicolon-delimited)
/// - Dates: DD.MM.YYYY (dot-separated)
/// - Amounts: Polish decimal format (-1180,00)
/// - Some fields have a leading single-quote prefix
pub fn parse(content: &str) -> Result<Vec<ParsedTransaction>, String> {
    let cleaned = content.replace('\r', "");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::Fields)
        .from_reader(cleaned.as_bytes());

    let mut transactions = Vec::new();

    for (row_idx, result) in reader.records().enumerate() {
        let record = result.map_err(|e| format!("CSV parse error at row {}: {}", row_idx + 2, e))?;

        if record.len() < MIN_COLUMNS {
            tracing::warn!(
                "Pekao row {} has only {} columns, expected {}, skipping",
                row_idx + 2,
                record.len(),
                MIN_COLUMNS
            );
            continue;
        }

        // Transaction date is "Data waluty" (index 1)
        let date_str = record.get(COL_DATA_WALUTY).unwrap_or("").trim();
        if date_str.is_empty() {
            continue;
        }

        let transaction_date = NaiveDate::parse_from_str(date_str, "%d.%m.%Y")
            .map_err(|e| format!("Row {}: invalid transaction date '{}': {}", row_idx + 2, date_str, e))?;

        // Booking date is "Data księgowania" (index 0)
        let booking_date_str = record.get(COL_DATA_KSIEGOWANIA).unwrap_or("").trim();
        let booking_date = if booking_date_str.is_empty() {
            None
        } else {
            Some(NaiveDate::parse_from_str(booking_date_str, "%d.%m.%Y")
                .map_err(|e| format!("Row {}: invalid booking date '{}': {}", row_idx + 2, booking_date_str, e))?)
        };

        let counterparty_raw = common::normalize_whitespace(
            record.get(COL_NADAWCA_ODBIORCA).unwrap_or("").trim(),
        );
        let tytul = common::normalize_whitespace(
            record.get(COL_TYTUL).unwrap_or("").trim(),
        );

        // Parse amount
        let amount_str = record.get(COL_KWOTA).unwrap_or("").trim();
        let amount = common::parse_polish_decimal(amount_str)
            .map_err(|e| format!("Row {}: {}", row_idx + 2, e))?;

        let currency = record.get(COL_WALUTA).unwrap_or("PLN").trim().to_string();

        // Counterparty
        let counterparty = if !counterparty_raw.is_empty() {
            Some(counterparty_raw.clone())
        } else {
            None
        };

        // Description: title, fallback to counterparty
        let description = if !tytul.is_empty() {
            tytul.clone()
        } else if !counterparty_raw.is_empty() {
            counterparty_raw.clone()
        } else {
            String::new()
        };

        // Bank category (index 11)
        let kategoria = record.get(COL_KATEGORIA).unwrap_or("").trim();
        let bank_category = if kategoria.is_empty() {
            None
        } else {
            Some(common::normalize_whitespace(kategoria))
        };

        // Bank reference (index 9), strip leading quote
        let ref_raw = record.get(COL_NUMER_REFERENCYJNY).unwrap_or("").trim();
        let ref_clean = strip_leading_quote(ref_raw).trim();
        let bank_reference = if ref_clean.is_empty() {
            None
        } else {
            Some(ref_clean.to_string())
        };

        // Bank type (index 10)
        let typ_raw = record.get(COL_TYP_OPERACJI).unwrap_or("").trim();
        let bank_type = if typ_raw.is_empty() {
            None
        } else {
            Some(common::normalize_whitespace(typ_raw))
        };

        let raw_data = json!({
            "data_ksiegowania": booking_date_str,
            "data_waluty": date_str,
            "nadawca_odbiorca": &counterparty_raw,
            "adres_nadawcy_odbiorcy": record.get(COL_ADRES).unwrap_or("").trim(),
            "rachunek_zrodlowy": strip_leading_quote(record.get(COL_RACHUNEK_ZRODLOWY).unwrap_or("").trim()),
            "rachunek_docelowy": strip_leading_quote(record.get(COL_RACHUNEK_DOCELOWY).unwrap_or("").trim()),
            "tytul": &tytul,
            "kwota_operacji": amount_str,
            "waluta": &currency,
            "numer_referencyjny": ref_clean,
            "typ_operacji": typ_raw,
            "kategoria": kategoria,
            "mile_transakcyjne": record.get(COL_MILE).unwrap_or("").trim(),
        });

        transactions.push(ParsedTransaction {
            account: "pekao".to_string(),
            transaction_date,
            booking_date,
            counterparty,
            description,
            amount,
            currency,
            bank_category,
            bank_reference,
            bank_type,
            state: "completed".to_string(),
            raw_data,
        });
    }

    if transactions.is_empty() {
        return Err("No valid transactions found in Pekao CSV".into());
    }

    tracing::info!("Parsed {} Pekao transactions", transactions.len());
    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn sample_csv() -> &'static str {
        "Data księgowania;Data waluty;Nadawca / Odbiorca;Adres nadawcy / odbiorcy;Rachunek Źródłowy;Rachunek docelowy;Tytuł;Kwota operacji;Waluta;Numer referencyjny;Typ operacji;Kategoria;Mile transakcyjne\n\
         23.02.2026;23.02.2026;Fitness Club;ul. Sportowa 1;'52470000123456789012345678;'11222233334444555566667777;Za treningi;-1180,00;PLN;'0001234567;Przelew wychodzący;Sport i rekreacja;0\n\
         22.02.2026;22.02.2026;Jan Kowalski;ul. Główna 5;'11222233334444555566667777;'52470000123456789012345678;Wynagrodzenie;5000,00;PLN;'0009876543;Przelew przychodzący;Wynagrodzenie;0\n\
         21.02.2026;21.02.2026;Sklep Biedronka;;'52470000123456789012345678;;Zakupy spożywcze;-49,47;PLN;'0005551234;Płatność kartą;Żywność;10\n"
    }

    #[test]
    fn test_parse_pekao_basic() {
        let result = parse(sample_csv()).expect("should parse successfully");
        assert_eq!(result.len(), 3);

        // First row: expense
        assert_eq!(result[0].account, "pekao");
        assert_eq!(result[0].transaction_date, NaiveDate::from_ymd_opt(2026, 2, 23).unwrap());
        assert_eq!(result[0].booking_date, Some(NaiveDate::from_ymd_opt(2026, 2, 23).unwrap()));
        assert_eq!(result[0].amount, Decimal::from_str("-1180.00").unwrap());
        assert_eq!(result[0].currency, "PLN");
        assert_eq!(result[0].state, "completed");
        assert_eq!(result[0].counterparty.as_deref(), Some("Fitness Club"));
        assert_eq!(result[0].description, "Za treningi");
        assert_eq!(result[0].bank_category.as_deref(), Some("Sport i rekreacja"));
        assert_eq!(result[0].bank_reference.as_deref(), Some("0001234567"));
        assert_eq!(result[0].bank_type.as_deref(), Some("Przelew wychodzący"));

        // Second row: income
        assert_eq!(result[1].amount, Decimal::from_str("5000.00").unwrap());
        assert_eq!(result[1].counterparty.as_deref(), Some("Jan Kowalski"));
        assert_eq!(result[1].description, "Wynagrodzenie");
        assert_eq!(result[1].bank_category.as_deref(), Some("Wynagrodzenie"));

        // Third row: card payment
        assert_eq!(result[2].amount, Decimal::from_str("-49.47").unwrap());
        assert_eq!(result[2].counterparty.as_deref(), Some("Sklep Biedronka"));
        assert_eq!(result[2].bank_type.as_deref(), Some("Płatność kartą"));
    }

    #[test]
    fn test_strip_leading_quote() {
        assert_eq!(strip_leading_quote("'52470000"), "52470000");
        assert_eq!(strip_leading_quote("52470000"), "52470000");
        assert_eq!(strip_leading_quote(""), "");
        assert_eq!(strip_leading_quote("'"), "");
    }

    #[test]
    fn test_parse_pekao_empty_title_fallback() {
        let csv = "Data księgowania;Data waluty;Nadawca / Odbiorca;Adres nadawcy / odbiorcy;Rachunek Źródłowy;Rachunek docelowy;Tytuł;Kwota operacji;Waluta;Numer referencyjny;Typ operacji;Kategoria;Mile transakcyjne\n\
                   23.02.2026;23.02.2026;Sklep ABC;;;;; -25,00;PLN;;;;\n";
        let result = parse(csv).expect("should parse");
        assert_eq!(result[0].description, "Sklep ABC");
        assert_eq!(result[0].counterparty.as_deref(), Some("Sklep ABC"));
    }

    #[test]
    fn test_parse_pekao_empty() {
        let csv = "Data księgowania;Data waluty;Nadawca / Odbiorca;Adres nadawcy / odbiorcy;Rachunek Źródłowy;Rachunek docelowy;Tytuł;Kwota operacji;Waluta;Numer referencyjny;Typ operacji;Kategoria;Mile transakcyjne\n";
        let result = parse(csv);
        assert!(result.is_err());
    }
}
