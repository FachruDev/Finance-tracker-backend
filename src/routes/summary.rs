use actix_web::{get, web, HttpResponse};
use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::category::CategoryBreakdownItem;

#[derive(Debug, Serialize)]
pub struct MonthlySummary {
    pub year: i32,
    pub month: u32,
    pub total_income: Decimal,
    pub total_expense: Decimal,
    pub balance: Decimal,
    pub category_breakdown: Vec<CategoryBreakdownItem>,
}

#[get("/summary/month")]
pub async fn monthly_summary(
    pool: web::Data<DbPool>,
    user: AuthUser,
    query: web::Query<SummaryQuery>,
) -> Result<HttpResponse, AppError> {
    let year = query.year;
    let month = query.month;
    let start = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| AppError::BadRequest("Invalid year/month".into()))?;
    let end = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    // totals
    let (total_income, total_expense): (Option<Decimal>, Option<Decimal>) = sqlx::query_as(
        r#"
        SELECT 
            SUM(CASE WHEN c.kind='income' THEN t.amount ELSE 0 END) AS income,
            SUM(CASE WHEN c.kind='expense' THEN t.amount ELSE 0 END) AS expense
        FROM transactions t
        JOIN categories c ON c.id = t.category_id
        WHERE t.user_id=$1 AND t.occurred_on >= $2 AND t.occurred_on < $3
        "#,
    )
    .bind(user.0)
    .bind(start)
    .bind(end)
    .fetch_one(pool.get_ref())
    .await?;

    let total_income = total_income.unwrap_or(Decimal::ZERO);
    let total_expense = total_expense.unwrap_or(Decimal::ZERO);
    let balance = total_income - total_expense;

    // breakdown
    let breakdown = sqlx::query_as::<_, CategoryBreakdownItem>(
        r#"
        SELECT t.category_id as category_id, c.name as name, c.kind as kind, COALESCE(SUM(t.amount),0) as total
        FROM transactions t
        JOIN categories c ON c.id = t.category_id
        WHERE t.user_id=$1 AND t.occurred_on >= $2 AND t.occurred_on < $3
        GROUP BY t.category_id, c.name, c.kind
        ORDER BY total DESC
        "#,
    )
    .bind(user.0)
    .bind(start)
    .bind(end)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(MonthlySummary {
        year,
        month,
        total_income,
        total_expense,
        balance,
        category_breakdown: breakdown,
    }))
}

#[derive(Debug,serde::Deserialize)]
pub struct SummaryQuery {
    pub year: i32,
    pub month: u32,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(monthly_summary);
}

