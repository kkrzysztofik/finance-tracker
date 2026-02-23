use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub hash: String,
    pub account_id: i32,
    pub transaction_date: NaiveDate,
    pub booking_date: Option<NaiveDate>,
    pub counterparty: Option<String>,
    pub description: String,
    pub amount: Decimal,
    pub currency: String,
    pub category_id: Option<i32>,
    pub category_source: Option<String>,
    pub bank_category: Option<String>,
    pub bank_reference: Option<String>,
    pub bank_type: Option<String>,
    pub state: Option<String>,
    pub raw_data: Option<serde_json::Value>,
    pub imported_at: Option<chrono::DateTime<chrono::Utc>>,
}
