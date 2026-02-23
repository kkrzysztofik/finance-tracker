use axum::extract::{Query, State};
use axum::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::error::AppError;

// --- Monthly ---

#[derive(Debug, Deserialize)]
pub struct MonthlyParams {
    pub account: Option<String>,
    pub year: Option<i32>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MonthlyRow {
    pub month: Option<String>,
    pub income: Option<Decimal>,
    pub expense: Option<Decimal>,
}

pub async fn monthly(
    State(pool): State<PgPool>,
    Query(params): Query<MonthlyParams>,
) -> Result<Json<Vec<MonthlyRow>>, AppError> {
    let mut conditions: Vec<String> = Vec::new();
    let mut bind_idx = 0u32;

    if params.account.is_some() {
        bind_idx += 1;
        conditions.push(format!("a.name = ${bind_idx}"));
    }
    if params.year.is_some() {
        bind_idx += 1;
        conditions.push(format!("EXTRACT(YEAR FROM t.transaction_date) = ${bind_idx}"));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT TO_CHAR(t.transaction_date, 'YYYY-MM') AS month, \
         SUM(CASE WHEN t.amount > 0 THEN t.amount ELSE 0 END) AS income, \
         SUM(CASE WHEN t.amount < 0 THEN t.amount ELSE 0 END) AS expense \
         FROM transactions t \
         JOIN accounts a ON t.account_id = a.id \
         {where_clause} \
         GROUP BY month \
         ORDER BY month"
    );

    let mut q = sqlx::query_as::<_, MonthlyRow>(&sql);
    if let Some(ref account) = params.account {
        q = q.bind(account);
    }
    if let Some(year) = params.year {
        q = q.bind(year);
    }

    let rows = q.fetch_all(&pool).await?;
    Ok(Json(rows))
}

// --- Category breakdown ---

#[derive(Debug, Deserialize)]
pub struct CategoryParams {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub account: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CategoryRow {
    pub category: Option<String>,
    pub total: Option<Decimal>,
    pub count: Option<i64>,
}

pub async fn by_category(
    State(pool): State<PgPool>,
    Query(params): Query<CategoryParams>,
) -> Result<Json<Vec<CategoryRow>>, AppError> {
    let mut conditions: Vec<String> = vec!["t.amount < 0".into()];
    let mut bind_idx = 0u32;

    if params.account.is_some() {
        bind_idx += 1;
        conditions.push(format!("a.name = ${bind_idx}"));
    }
    if params.date_from.is_some() {
        bind_idx += 1;
        conditions.push(format!("t.transaction_date >= ${bind_idx}::date"));
    }
    if params.date_to.is_some() {
        bind_idx += 1;
        conditions.push(format!("t.transaction_date <= ${bind_idx}::date"));
    }

    let where_clause = format!("WHERE {}", conditions.join(" AND "));

    let sql = format!(
        "SELECT c.name AS category, SUM(t.amount) AS total, COUNT(t.id) AS count \
         FROM transactions t \
         JOIN accounts a ON t.account_id = a.id \
         LEFT JOIN categories c ON t.category_id = c.id \
         {where_clause} \
         GROUP BY c.name \
         ORDER BY total ASC"
    );

    let mut q = sqlx::query_as::<_, CategoryRow>(&sql);
    if let Some(ref account) = params.account {
        q = q.bind(account);
    }
    if let Some(ref date_from) = params.date_from {
        q = q.bind(date_from);
    }
    if let Some(ref date_to) = params.date_to {
        q = q.bind(date_to);
    }

    let rows = q.fetch_all(&pool).await?;
    Ok(Json(rows))
}
