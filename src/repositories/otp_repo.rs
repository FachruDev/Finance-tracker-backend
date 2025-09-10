use crate::db::DbPool;
use crate::errors::AppError;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::{self, Executor, Postgres};

pub async fn last_created_at(pool: &DbPool, user_id: Uuid, purpose: &str) -> Result<Option<DateTime<Utc>>, AppError> {
    let row: Option<(DateTime<Utc>,)> = sqlx::query_as(
        "SELECT created_at FROM user_otp_codes WHERE user_id=$1 AND purpose=$2 AND used_at IS NULL ORDER BY created_at DESC LIMIT 1",
    )
    .bind(user_id)
    .bind(purpose)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn create(pool: &DbPool, user_id: Uuid, code: &str, purpose: &str, expires_at: DateTime<Utc>) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO user_otp_codes (id, user_id, code, expires_at, purpose) VALUES ($1,$2,$3,$4,$5)",
    )
    .bind(id)
    .bind(user_id)
    .bind(code)
    .bind(expires_at)
    .bind(purpose)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_valid(pool: &DbPool, user_id: Uuid, code: &str, purpose: &str) -> Result<Option<Uuid>, AppError> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM user_otp_codes WHERE user_id=$1 AND code=$2 AND purpose=$3 AND used_at IS NULL AND expires_at > now() ORDER BY created_at DESC LIMIT 1",
    )
    .bind(user_id)
    .bind(code)
    .bind(purpose)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn mark_used<'e, E>(exec: E, otp_id: Uuid) -> Result<(), AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE user_otp_codes SET used_at = now() WHERE id=$1")
        .bind(otp_id)
        .execute(exec)
        .await?;
    Ok(())
}
