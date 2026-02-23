use rust_decimal::Decimal;
use sha2::{Digest, Sha256};
use std::str::FromStr;

/// Parse Polish decimal format: "1 234,56" or "-1234,56" → Decimal
pub fn parse_polish_decimal(s: &str) -> Result<Decimal, String> {
    let cleaned: String = s
        .trim()
        .replace('\u{a0}', "") // non-breaking space
        .replace(' ', "")
        .replace(',', ".");

    if cleaned.is_empty() {
        return Err("Empty decimal string".into());
    }

    Decimal::from_str(&cleaned).map_err(|e| format!("Invalid decimal '{}': {}", s, e))
}

/// Fix mojibake from double-encoded UTF-8 (UTF-8 → Latin-1 → UTF-8).
/// Revolut exports suffer from this: "ę" (c4 99) becomes "Ä™" (c3 84 c2 99).
pub fn fix_mojibake(s: &str) -> String {
    // Try to reverse the double-encoding
    match s
        .as_bytes()
        .iter()
        .copied()
        .collect::<Vec<u8>>()
        .as_slice()
    {
        bytes => {
            // Attempt: interpret as UTF-8, encode to Latin-1, decode as UTF-8
            if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                let latin1_bytes: Result<Vec<u8>, _> = text
                    .chars()
                    .map(|c| {
                        let cp = c as u32;
                        if cp <= 255 {
                            Ok(cp as u8)
                        } else {
                            Err(())
                        }
                    })
                    .collect();

                if let Ok(latin1) = latin1_bytes {
                    if let Ok(fixed) = String::from_utf8(latin1) {
                        // Verify it actually changed and looks valid
                        if fixed != text && fixed.chars().all(|c| !c.is_control() || c == '\n' || c == '\r' || c == '\t') {
                            return fixed;
                        }
                    }
                }
            }
            s.to_string()
        }
    }
}

/// Compute SHA-256 hash for deduplication: "{account}|{date_iso}|{amount}|{normalized_description}"
pub fn compute_hash(account: &str, date: &str, amount: &str, description: &str) -> String {
    let normalized_desc = description
        .trim()
        .to_lowercase()
        .replace("  ", " ");

    let input = format!("{}|{}|{}|{}", account, date, amount, normalized_desc);

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

/// Normalize whitespace in a string
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_polish_decimal() {
        assert_eq!(parse_polish_decimal("-1180,00").unwrap(), Decimal::from_str("-1180.00").unwrap());
        assert_eq!(parse_polish_decimal("340,00").unwrap(), Decimal::from_str("340.00").unwrap());
        assert_eq!(parse_polish_decimal("-49,47").unwrap(), Decimal::from_str("-49.47").unwrap());
        assert_eq!(parse_polish_decimal("-7,19").unwrap(), Decimal::from_str("-7.19").unwrap());
        assert_eq!(parse_polish_decimal("50").unwrap(), Decimal::from_str("50").unwrap());
        assert_eq!(parse_polish_decimal("-11").unwrap(), Decimal::from_str("-11").unwrap());
    }

    #[test]
    fn test_compute_hash_deterministic() {
        let h1 = compute_hash("alior", "2026-02-23", "-1180.00", "Za treningi");
        let h2 = compute_hash("alior", "2026-02-23", "-1180.00", "Za treningi");
        assert_eq!(h1, h2);
    }
}
