use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::transaction::Transaction;
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;

pub async fn list(
    pool: &DbPool,
    user_id: Uuid,
    category_id: Option<Uuid>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<Vec<Transaction>, AppError> {
    let rows = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, category_id, amount, occurred_on, description
        FROM transactions
        WHERE user_id = $1
          AND ($2::uuid IS NULL OR category_id = $2)
          AND ($3::date IS NULL OR occurred_on >= $3)
          AND ($4::date IS NULL OR occurred_on <= $4)
        ORDER BY occurred_on DESC, id DESC
        "#,
    )
    .bind(user_id)
    .bind(category_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_by_id_user(pool: &DbPool, id: Uuid, user_id: Uuid) -> Result<Option<Transaction>, AppError> {
    let row = sqlx::query_as::<_, Transaction>(
        "SELECT id, user_id, category_id, amount, occurred_on, description FROM transactions WHERE id=$1 AND user_id=$2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert(
    pool: &DbPool,
    id: Uuid,
    user_id: Uuid,
    category_id: Uuid,
    amount: &Decimal,
    occurred_on: NaiveDate,
    description: &Option<String>,
) -> Result<Transaction, AppError> {
    let row = sqlx::query_as::<_, Transaction>(
        r#"INSERT INTO transactions (id, user_id, category_id, amount, occurred_on, description)
            VALUES ($1,$2,$3,$4,$5,$6)
            RETURNING id, user_id, category_id, amount, occurred_on, description"#,
    )
    .bind(id)
    .bind(user_id)
    .bind(category_id)
    .bind(amount)
    .bind(occurred_on)
    .bind(description)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update(
    pool: &DbPool,
    id: Uuid,
    user_id: Uuid,
    category_id: Uuid,
    amount: &Decimal,
    occurred_on: NaiveDate,
    description: &Option<String>,
) -> Result<Transaction, AppError> {
    let row = sqlx::query_as::<_, Transaction>(
        r#"UPDATE transactions SET category_id=$1, amount=$2, occurred_on=$3, description=$4
           WHERE id=$5 AND user_id=$6
           RETURNING id, user_id, category_id, amount, occurred_on, description"#,
    )
    .bind(category_id)
    .bind(amount)
    .bind(occurred_on)
    .bind(description)
    .bind(id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn delete(pool: &DbPool, id: Uuid, user_id: Uuid) -> Result<u64, AppError> {
    let res = sqlx::query("DELETE FROM transactions WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

