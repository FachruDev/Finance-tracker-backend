use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::category::Category;
use uuid::Uuid;

pub async fn list_by_user(pool: &DbPool, user_id: Uuid) -> Result<Vec<Category>, AppError> {
    let rows = sqlx::query_as::<_, Category>(
        "SELECT id, user_id, name, kind, color, created_at FROM categories WHERE user_id=$1 ORDER BY name",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_by_id_user(pool: &DbPool, id: Uuid, user_id: Uuid) -> Result<Option<Category>, AppError> {
    let row = sqlx::query_as::<_, Category>(
        "SELECT id, user_id, name, kind, color, created_at FROM categories WHERE id=$1 AND user_id=$2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert(pool: &DbPool, id: Uuid, user_id: Uuid, name: &str, kind: &str, color: &str) -> Result<Category, AppError> {
    let row = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (id, user_id, name, kind, color)
         VALUES ($1,$2,$3,$4,$5)
         RETURNING id, user_id, name, kind, color, created_at",
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(kind)
    .bind(color)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update(pool: &DbPool, id: Uuid, user_id: Uuid, name: &str, kind: &str, color: &str) -> Result<Category, AppError> {
    let row = sqlx::query_as::<_, Category>(
        "UPDATE categories SET name=$1, kind=$2, color=$3 WHERE id=$4 AND user_id=$5
         RETURNING id, user_id, name, kind, color, created_at",
    )
    .bind(name)
    .bind(kind)
    .bind(color)
    .bind(id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn delete(pool: &DbPool, id: Uuid, user_id: Uuid) -> Result<u64, AppError> {
    let res = sqlx::query("DELETE FROM categories WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

