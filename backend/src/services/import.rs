use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    sea_query::OnConflict,
};
use tracing::info;

use crate::entities::{accounts, import_logs, transactions};
use crate::parsers;
use crate::parsers::common::compute_hash;

pub struct ImportResult {
    pub total_rows: i32,
    pub imported: i32,
    pub skipped: i32,
}

pub async fn import_file(
    db: &DatabaseConnection,
    filename: &str,
    content: &str,
) -> Result<ImportResult, String> {
    let parsed = parsers::detect_and_parse(filename, content)?;
    let total_rows = parsed.len() as i32;

    let mut imported = 0i32;
    let mut skipped = 0i32;

    for tx in &parsed {
        let account = accounts::Entity::find()
            .filter(accounts::Column::Name.eq(&tx.account))
            .one(db)
            .await
            .map_err(|e| format!("Account '{}' lookup error: {}", tx.account, e))?
            .ok_or_else(|| format!("Account '{}' not found", tx.account))?;

        let hash = compute_hash(
            &tx.account,
            &tx.transaction_date.to_string(),
            &tx.amount.to_string(),
            &tx.description,
        );

        let model = transactions::ActiveModel {
            hash: Set(hash),
            account_id: Set(account.id),
            transaction_date: Set(tx.transaction_date),
            booking_date: Set(tx.booking_date),
            counterparty: Set(tx.counterparty.clone()),
            description: Set(tx.description.clone()),
            amount: Set(tx.amount),
            currency: Set(tx.currency.clone()),
            category_source: Set(tx.bank_category.as_ref().map(|_| "bank".to_string())),
            bank_category: Set(tx.bank_category.clone()),
            bank_reference: Set(tx.bank_reference.clone()),
            bank_type: Set(tx.bank_type.clone()),
            state: Set(Some(tx.state.clone())),
            raw_data: Set(Some(tx.raw_data.clone())),
            ..Default::default()
        };

        let result = transactions::Entity::insert(model)
            .on_conflict(
                OnConflict::column(transactions::Column::Hash)
                    .do_nothing()
                    .to_owned(),
            )
            .exec_without_returning(db)
            .await;

        match result {
            Ok(_) => imported += 1,
            Err(sea_orm::DbErr::RecordNotInserted) => skipped += 1,
            Err(e) => return Err(format!("Insert error: {}", e)),
        }
    }

    // Log the import
    let account_name = parsed
        .first()
        .map(|t| t.account.as_str())
        .unwrap_or("unknown");
    let account = accounts::Entity::find()
        .filter(accounts::Column::Name.eq(account_name))
        .one(db)
        .await
        .map_err(|e| format!("Account lookup error: {}", e))?
        .ok_or_else(|| format!("Account '{}' not found", account_name))?;

    let log = import_logs::ActiveModel {
        filename: Set(filename.to_string()),
        account_id: Set(account.id),
        total_rows: Set(total_rows),
        imported: Set(imported),
        skipped: Set(skipped),
        ..Default::default()
    };
    log.insert(db)
        .await
        .map_err(|e| format!("Import log error: {}", e))?;

    info!("Import complete: {total_rows} total, {imported} imported, {skipped} skipped (duplicates)");

    Ok(ImportResult {
        total_rows,
        imported,
        skipped,
    })
}
