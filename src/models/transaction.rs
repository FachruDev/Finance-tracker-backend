use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub amount: Decimal,
    pub occurred_on: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransaction {
    pub category_id: Uuid,
    pub amount: Decimal,
    pub occurred_on: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransaction {
    pub category_id: Option<Uuid>,
    pub amount: Option<Decimal>,
    pub occurred_on: Option<NaiveDate>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TxnQuery {
    pub category_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

