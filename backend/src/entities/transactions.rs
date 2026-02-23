use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub hash: String,
    pub account_id: i32,
    pub transaction_date: chrono::NaiveDate,
    pub booking_date: Option<chrono::NaiveDate>,
    pub counterparty: Option<String>,
    pub description: String,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub category_id: Option<i32>,
    pub category_source: Option<String>,
    pub bank_category: Option<String>,
    pub bank_reference: Option<String>,
    pub bank_type: Option<String>,
    pub state: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub raw_data: Option<serde_json::Value>,
    pub imported_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::AccountId",
        to = "super::accounts::Column::Id"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id"
    )]
    Category,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
