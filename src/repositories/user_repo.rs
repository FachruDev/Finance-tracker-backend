use crate::models::user::User;
use crate::db::DbPool;
use crate::errors::AppError;
use uuid::Uuid;

pub async fn get_by_email(pool: &DbPool, email: &str) -> Result<Option<User>, AppError> {
    let u = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, auth_provider, google_sub, is_verified, created_at FROM users WHERE email=$1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(u)
}

pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<User, AppError> {
    let u = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, auth_provider, google_sub, is_verified, created_at FROM users WHERE id=$1",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(u)
}

pub async fn insert_local(pool: &DbPool, id: Uuid, name: &str, email: &str, password_hash: &str) -> Result<User, AppError> {
    let u = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email, password_hash, auth_provider, google_sub, is_verified)
         VALUES ($1,$2,$3,$4,'local', NULL, false)
         RETURNING id, name, email, password_hash, auth_provider, google_sub, is_verified, created_at",
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;
    Ok(u)
}

pub async fn insert_google(pool: &DbPool, id: Uuid, name: &str, email: &str, sub: &str) -> Result<User, AppError> {
    let u = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email, password_hash, auth_provider, google_sub, is_verified)
         VALUES ($1,$2,$3,'', 'google', $4, false)
         RETURNING id, name, email, password_hash, auth_provider, google_sub, is_verified, created_at",
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .bind(sub)
    .fetch_one(pool)
    .await?;
    Ok(u)
}

pub async fn link_google_sub(pool: &DbPool, id: Uuid, sub: &str) -> Result<User, AppError> {
    let u = sqlx::query_as::<_, User>(
        "UPDATE users SET google_sub=$1, auth_provider='google' WHERE id=$2
         RETURNING id, name, email, password_hash, auth_provider, google_sub, is_verified, created_at",
    )
    .bind(sub)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(u)
}

// Note: set_verified and set_password moved into services.

pub async fn delete_by_id(pool: &DbPool, id: Uuid) -> Result<u64, AppError> {
    let res = sqlx::query("DELETE FROM users WHERE id=$1").bind(id).execute(pool).await?;
    Ok(res.rows_affected())
}
