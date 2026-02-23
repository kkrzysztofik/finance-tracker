use axum::extract::State;
use axum::Json;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryOrder,
    QuerySelect, RelationTrait,
};
use serde::Serialize;

use crate::entities::{accounts, transactions};
use crate::error::AppError;

#[derive(Debug, Serialize, FromQueryResult)]
pub struct AccountWithCount {
    pub id: i32,
    pub name: String,
    pub currency: String,
    pub transaction_count: i64,
}

pub async fn list(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<AccountWithCount>>, AppError> {
    let accounts = accounts::Entity::find()
        .select_only()
        .column(accounts::Column::Id)
        .column(accounts::Column::Name)
        .column(accounts::Column::Currency)
        .column_as(transactions::Column::Id.count(), "transaction_count")
        .join(JoinType::LeftJoin, accounts::Relation::Transactions.def())
        .group_by(accounts::Column::Id)
        .group_by(accounts::Column::Name)
        .group_by(accounts::Column::Currency)
        .order_by_asc(accounts::Column::Name)
        .into_model::<AccountWithCount>()
        .all(&db)
        .await?;

    Ok(Json(accounts))
}
