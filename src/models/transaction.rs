use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;
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

// DTOs moved to src/dto/transaction.rs
