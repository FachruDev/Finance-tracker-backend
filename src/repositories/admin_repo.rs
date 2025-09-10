use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::admin::Admin;
use uuid::Uuid;

pub async fn count_admins(pool: &DbPool) -> Result<i64, AppError> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(1) FROM admins")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

pub async fn get_by_email(pool: &DbPool, email: &str) -> Result<Option<Admin>, AppError> {
    let row = sqlx::query_as::<_, Admin>(
        "SELECT id, name, email, password_hash, created_at FROM admins WHERE email=$1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<Admin, AppError> {
    let row = sqlx::query_as::<_, Admin>(
        "SELECT id, name, email, password_hash, created_at FROM admins WHERE id=$1",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn insert(pool: &DbPool, id: Uuid, name: &str, email: &str, password_hash: &str) -> Result<Admin, AppError> {
    let row = sqlx::query_as::<_, Admin>(
        "INSERT INTO admins (id, name, email, password_hash) VALUES ($1,$2,$3,$4)
         RETURNING id, name, email, password_hash, created_at",
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

