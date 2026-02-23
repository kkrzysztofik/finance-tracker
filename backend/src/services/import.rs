use sqlx::PgPool;
use tracing::info;

use crate::parsers::{self, ParsedTransaction};
use crate::parsers::common::compute_hash;

pub struct ImportResult {
    pub total_rows: i32,
    pub imported: i32,
    pub skipped: i32,
}

pub async fn import_file(pool: &PgPool, filename: &str, content: &str) -> Result<ImportResult, String> {
    let transactions = parsers::detect_and_parse(filename, content)?;
    let total_rows = transactions.len() as i32;

    // Get account ID mapping
    let mut imported = 0i32;
    let mut skipped = 0i32;

    for tx in &transactions {
        let account_id: i32 = sqlx::query_scalar("SELECT id FROM accounts WHERE name = $1")
            .bind(&tx.account)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Account '{}' not found: {}", tx.account, e))?;

        let hash = compute_hash(
            &tx.account,
            &tx.transaction_date.to_string(),
            &tx.amount.to_string(),
            &tx.description,
        );

        let result = sqlx::query(
            "INSERT INTO transactions (hash, account_id, transaction_date, booking_date, counterparty, description, amount, currency, category_source, bank_category, bank_reference, bank_type, state, raw_data)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
             ON CONFLICT (hash) DO NOTHING"
        )
        .bind(&hash)
        .bind(account_id)
        .bind(tx.transaction_date)
        .bind(tx.booking_date)
        .bind(&tx.counterparty)
        .bind(&tx.description)
        .bind(tx.amount)
        .bind(&tx.currency)
        .bind(tx.bank_category.as_ref().map(|_| "bank"))
        .bind(&tx.bank_category)
        .bind(&tx.bank_reference)
        .bind(&tx.bank_type)
        .bind(&tx.state)
        .bind(&tx.raw_data)
        .execute(pool)
        .await
        .map_err(|e| format!("Insert error: {}", e))?;

        if result.rows_affected() > 0 {
            imported += 1;
        } else {
            skipped += 1;
        }
    }

    // Log the import
    let account_name = transactions.first().map(|t| t.account.as_str()).unwrap_or("unknown");
    let account_id: i32 = sqlx::query_scalar("SELECT id FROM accounts WHERE name = $1")
        .bind(account_name)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Account lookup error: {}", e))?;

    sqlx::query(
        "INSERT INTO import_logs (filename, account_id, total_rows, imported, skipped) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(filename)
    .bind(account_id)
    .bind(total_rows)
    .bind(imported)
    .bind(skipped)
    .execute(pool)
    .await
    .map_err(|e| format!("Import log error: {}", e))?;

    info!("Import complete: {total_rows} total, {imported} imported, {skipped} skipped (duplicates)");

    Ok(ImportResult { total_rows, imported, skipped })
}
