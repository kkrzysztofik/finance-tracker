use axum::extract::{Json, Path, Query, State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::Transaction;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub account: Option<String>,
    pub category_id: Option<i32>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub data: Vec<Transaction>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Deserialize)]
pub struct UpdateCategoryBody {
    pub category_id: Option<i32>,
}

pub async fn list(
    State(pool): State<PgPool>,
    Query(params): Query<ListParams>,
) -> Result<Json<ListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).clamp(1, 200);
    let offset = (page - 1) * per_page;

    let sort_by = match params.sort_by.as_deref() {
        Some("amount") => "t.amount",
        Some("description") => "t.description",
        Some("counterparty") => "t.counterparty",
        Some("imported_at") => "t.imported_at",
        _ => "t.transaction_date",
    };
    let sort_order = match params.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Build dynamic WHERE clauses
    let mut conditions: Vec<String> = Vec::new();
    let mut bind_idx = 0u32;

    // We'll collect bind values and apply them positionally.
    // Since sqlx doesn't support dynamic binds easily with query_as,
    // we build the full SQL string with numbered params and use query_as with raw SQL.

    if params.account.is_some() {
        bind_idx += 1;
        conditions.push(format!("a.name = ${bind_idx}"));
    }
    if params.category_id.is_some() {
        bind_idx += 1;
        conditions.push(format!("t.category_id = ${bind_idx}"));
    }
    if params.date_from.is_some() {
        bind_idx += 1;
        conditions.push(format!("t.transaction_date >= ${bind_idx}::date"));
    }
    if params.date_to.is_some() {
        bind_idx += 1;
        conditions.push(format!("t.transaction_date <= ${bind_idx}::date"));
    }
    if params.search.is_some() {
        bind_idx += 1;
        conditions.push(format!(
            "(t.description ILIKE ${bind_idx} OR t.counterparty ILIKE ${bind_idx})"
        ));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_sql = format!(
        "SELECT COUNT(*) FROM transactions t JOIN accounts a ON t.account_id = a.id {where_clause}"
    );

    // Data query
    let data_sql = format!(
        "SELECT t.id, t.hash, t.account_id, t.transaction_date, t.booking_date, \
         t.counterparty, t.description, t.amount, t.currency, t.category_id, \
         t.category_source, t.bank_category, t.bank_reference, t.bank_type, \
         t.state, t.raw_data, t.imported_at \
         FROM transactions t JOIN accounts a ON t.account_id = a.id \
         {where_clause} ORDER BY {sort_by} {sort_order}, t.id DESC \
         LIMIT {per_page} OFFSET {offset}"
    );

    // Bind params in order
    macro_rules! bind_params {
        ($query:expr) => {{
            let mut q = $query;
            if let Some(ref account) = params.account {
                q = q.bind(account);
            }
            if let Some(category_id) = params.category_id {
                q = q.bind(category_id);
            }
            if let Some(ref date_from) = params.date_from {
                q = q.bind(date_from);
            }
            if let Some(ref date_to) = params.date_to {
                q = q.bind(date_to);
            }
            if let Some(ref search) = params.search {
                q = q.bind(format!("%{search}%"));
            }
            q
        }};
    }

    let total: i64 = {
        let q = sqlx::query_scalar::<_, i64>(&count_sql);
        bind_params!(q).fetch_one(&pool).await?
    };

    let data: Vec<Transaction> = {
        let q = sqlx::query_as::<_, Transaction>(&data_sql);
        bind_params!(q).fetch_all(&pool).await?
    };

    Ok(Json(ListResponse {
        data,
        total,
        page,
        per_page,
    }))
}

pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Transaction>, AppError> {
    let tx = sqlx::query_as::<_, Transaction>(
        "SELECT id, hash, account_id, transaction_date, booking_date, \
         counterparty, description, amount, currency, category_id, \
         category_source, bank_category, bank_reference, bank_type, \
         state, raw_data, imported_at \
         FROM transactions WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Transaction {id} not found")))?;

    Ok(Json(tx))
}

pub async fn update_category(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateCategoryBody>,
) -> Result<Json<Transaction>, AppError> {
    let tx = sqlx::query_as::<_, Transaction>(
        "UPDATE transactions SET category_id = $1, category_source = 'manual' \
         WHERE id = $2 \
         RETURNING id, hash, account_id, transaction_date, booking_date, \
         counterparty, description, amount, currency, category_id, \
         category_source, bank_category, bank_reference, bank_type, \
         state, raw_data, imported_at",
    )
    .bind(body.category_id)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Transaction {id} not found")))?;

    Ok(Json(tx))
}
