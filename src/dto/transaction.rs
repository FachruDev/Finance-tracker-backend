use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTransaction {
    pub category_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub occurred_on: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransaction {
    pub category_id: Option<Uuid>,
    pub amount: Option<rust_decimal::Decimal>,
    pub occurred_on: Option<NaiveDate>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TxnQuery {
    pub category_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

