use axum::extract::{Json, Path, Query, State};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

use crate::entities::{accounts, transactions};
use crate::error::AppError;
use crate::models::Transaction;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub account: Option<String>,
    pub category_id: Option<i32>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub data: Vec<Transaction>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Deserialize)]
pub struct UpdateCategoryBody {
    pub category_id: Option<i32>,
}

pub async fn list(
    State(db): State<DatabaseConnection>,
    Query(params): Query<ListParams>,
) -> Result<Json<ListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(50).clamp(1, 200);

    let mut condition = Condition::all();

    if let Some(ref account) = params.account {
        condition = condition.add(accounts::Column::Name.eq(account.as_str()));
    }
    if let Some(category_id) = params.category_id {
        condition = condition.add(transactions::Column::CategoryId.eq(category_id));
    }
    if let Some(ref date_from) = params.date_from {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(date_from, "%Y-%m-%d") {
            condition = condition.add(transactions::Column::TransactionDate.gte(d));
        }
    }
    if let Some(ref date_to) = params.date_to {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(date_to, "%Y-%m-%d") {
            condition = condition.add(transactions::Column::TransactionDate.lte(d));
        }
    }
    if let Some(ref search) = params.search {
        let pattern = format!("%{search}%");
        condition = condition.add(
            Condition::any()
                .add(transactions::Column::Description.like(&pattern))
                .add(transactions::Column::Counterparty.like(&pattern)),
        );
    }

    let base_query = transactions::Entity::find()
        .join(JoinType::InnerJoin, transactions::Relation::Account.def())
        .filter(condition.clone());

    let total = base_query.clone().count(&db).await?;

    let mut data_query = transactions::Entity::find()
        .join(JoinType::InnerJoin, transactions::Relation::Account.def())
        .filter(condition);

    // Apply sorting
    data_query = match params.sort_by.as_deref() {
        Some("amount") => match params.sort_order.as_deref() {
            Some("asc") => data_query.order_by_asc(transactions::Column::Amount),
            _ => data_query.order_by_desc(transactions::Column::Amount),
        },
        Some("description") => match params.sort_order.as_deref() {
            Some("asc") => data_query.order_by_asc(transactions::Column::Description),
            _ => data_query.order_by_desc(transactions::Column::Description),
        },
        Some("counterparty") => match params.sort_order.as_deref() {
            Some("asc") => data_query.order_by_asc(transactions::Column::Counterparty),
            _ => data_query.order_by_desc(transactions::Column::Counterparty),
        },
        Some("imported_at") => match params.sort_order.as_deref() {
            Some("asc") => data_query.order_by_asc(transactions::Column::ImportedAt),
            _ => data_query.order_by_desc(transactions::Column::ImportedAt),
        },
        _ => match params.sort_order.as_deref() {
            Some("asc") => data_query.order_by_asc(transactions::Column::TransactionDate),
            _ => data_query.order_by_desc(transactions::Column::TransactionDate),
        },
    };

    data_query = data_query.order_by_desc(transactions::Column::Id);

    let data = data_query
        .paginate(&db, per_page)
        .fetch_page(page - 1)
        .await?;

    Ok(Json(ListResponse {
        data,
        total,
        page,
        per_page,
    }))
}

pub async fn get_one(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<Transaction>, AppError> {
    let tx = transactions::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Transaction {id} not found")))?;

    Ok(Json(tx))
}

pub async fn update_category(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateCategoryBody>,
) -> Result<Json<Transaction>, AppError> {
    // Verify it exists
    let existing = transactions::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Transaction {id} not found")))?;

    let mut active: transactions::ActiveModel = existing.into();
    active.category_id = Set(body.category_id);
    active.category_source = Set(Some("manual".to_string()));

    let updated = active.update(&db).await?;
    Ok(Json(updated))
}
