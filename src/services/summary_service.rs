use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::routes::summary::MonthlySummary; // reuse struct
use crate::models::category::CategoryBreakdownItem;

pub async fn monthly_summary(pool: &DbPool, user_id: Uuid, year: i32, month: u32) -> Result<MonthlySummary, AppError> {
    let start = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| AppError::BadRequest("Invalid year/month".into()))?;
    let end = if month == 12 { NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() } else { NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() };

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
    .bind(user_id)
    .bind(start)
    .bind(end)
    .fetch_one(pool)
    .await?;

    let total_income = total_income.unwrap_or(Decimal::ZERO);
    let total_expense = total_expense.unwrap_or(Decimal::ZERO);
    let balance = total_income - total_expense;

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
    .bind(user_id)
    .bind(start)
    .bind(end)
    .fetch_all(pool)
    .await?;

    Ok(MonthlySummary { year, month, total_income, total_expense, balance, category_breakdown: breakdown })
}

