use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub kind: String, // "income" | "expense"
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub kind: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CategoryBreakdownItem {
    pub category_id: Uuid,
    pub name: String,
    pub kind: String,
    pub total: Decimal,
}
