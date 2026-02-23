use axum::extract::{Query, State};
use axum::Json;
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, JoinType,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use sea_orm::sea_query::{Alias, Expr, Func, SimpleExpr};
use serde::{Deserialize, Serialize};

use crate::entities::{accounts, categories, transactions};
use crate::error::AppError;

// --- Monthly ---

#[derive(Debug, Deserialize)]
pub struct MonthlyParams {
    pub account: Option<String>,
    pub year: Option<i32>,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct MonthlyRow {
    pub month: Option<String>,
    pub income: Option<Decimal>,
    pub expense: Option<Decimal>,
}

pub async fn monthly(
    State(db): State<DatabaseConnection>,
    Query(params): Query<MonthlyParams>,
) -> Result<Json<Vec<MonthlyRow>>, AppError> {
    let mut condition = Condition::all();

    if let Some(ref account) = params.account {
        condition = condition.add(accounts::Column::Name.eq(account.as_str()));
    }
    if let Some(year) = params.year {
        condition = condition.add(
            Expr::cust_with_expr(
                "EXTRACT(YEAR FROM $1)",
                Expr::col((transactions::Entity, transactions::Column::TransactionDate)),
            )
            .eq(year),
        );
    }

    let month_expr = SimpleExpr::FunctionCall(
        Func::cust(Alias::new("TO_CHAR")).args([
            Expr::col((transactions::Entity, transactions::Column::TransactionDate)).into(),
            Expr::val("YYYY-MM").into(),
        ]),
    );

    let income_expr = Expr::cust(
        "SUM(CASE WHEN transactions.amount > 0 THEN transactions.amount ELSE 0 END)",
    );
    let expense_expr = Expr::cust(
        "SUM(CASE WHEN transactions.amount < 0 THEN transactions.amount ELSE 0 END)",
    );

    let rows = transactions::Entity::find()
        .select_only()
        .column_as(month_expr.clone(), "month")
        .column_as(income_expr, "income")
        .column_as(expense_expr, "expense")
        .join(JoinType::InnerJoin, transactions::Relation::Account.def())
        .filter(condition)
        .group_by(month_expr)
        .order_by_asc(Expr::cust("month"))
        .into_model::<MonthlyRow>()
        .all(&db)
        .await?;

    Ok(Json(rows))
}

// --- Category breakdown ---

#[derive(Debug, Deserialize)]
pub struct CategoryParams {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub account: Option<String>,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct CategoryRow {
    pub category: Option<String>,
    pub total: Option<Decimal>,
    pub count: Option<i64>,
}

pub async fn by_category(
    State(db): State<DatabaseConnection>,
    Query(params): Query<CategoryParams>,
) -> Result<Json<Vec<CategoryRow>>, AppError> {
    let mut condition = Condition::all()
        .add(transactions::Column::Amount.lt(Decimal::ZERO));

    if let Some(ref account) = params.account {
        condition = condition.add(accounts::Column::Name.eq(account.as_str()));
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

    let rows = transactions::Entity::find()
        .select_only()
        .column_as(categories::Column::Name, "category")
        .column_as(transactions::Column::Amount.sum(), "total")
        .column_as(transactions::Column::Id.count(), "count")
        .join(JoinType::InnerJoin, transactions::Relation::Account.def())
        .join(JoinType::LeftJoin, transactions::Relation::Category.def())
        .filter(condition)
        .group_by(categories::Column::Name)
        .order_by_asc(Expr::cust("total"))
        .into_model::<CategoryRow>()
        .all(&db)
        .await?;

    Ok(Json(rows))
}
