use axum::extract::State;
use axum::Json;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

use crate::error::AppError;

#[derive(Debug, Serialize, FromRow)]
pub struct AccountWithCount {
    pub id: i32,
    pub name: String,
    pub currency: String,
    pub transaction_count: i64,
}

pub async fn list(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<AccountWithCount>>, AppError> {
    let accounts = sqlx::query_as::<_, AccountWithCount>(
        "SELECT a.id, a.name, a.currency, COUNT(t.id) AS transaction_count \
         FROM accounts a \
         LEFT JOIN transactions t ON t.account_id = a.id \
         GROUP BY a.id, a.name, a.currency \
         ORDER BY a.name",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(accounts))
}
