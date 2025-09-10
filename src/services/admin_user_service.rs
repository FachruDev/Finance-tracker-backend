use uuid::Uuid;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::user::{PublicUser, User};
use crate::repositories::user_repo as repo;
use crate::auth::hash_password;

#[derive(Debug, serde::Deserialize)]
pub struct CreateUserReq { pub name: String, pub email: String, pub password: String }

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserReq { pub name: Option<String>, pub email: Option<String>, pub password: Option<String> }

pub async fn list(pool: &DbPool) -> Result<Vec<PublicUser>, AppError> {
    let rows = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, auth_provider, google_sub, is_verified, created_at FROM users ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn get(pool: &DbPool, id: Uuid) -> Result<PublicUser, AppError> {
    Ok(repo::get_by_id(pool, id).await?.into())
}

pub async fn create(pool: &DbPool, payload: CreateUserReq) -> Result<PublicUser, AppError> {
    let id = Uuid::new_v4();
    let email = payload.email.trim().to_lowercase();
    let hash = hash_password(&payload.password)?;
    let rec = repo::insert_local(pool, id, &payload.name, &email, &hash).await
        .map_err(|e| match e { AppError::Db(s) if s.contains("unique") => AppError::Conflict("Email already registered".into()), other => other })?;
    Ok(rec.into())
}

pub async fn update(pool: &DbPool, id: Uuid, payload: UpdateUserReq) -> Result<PublicUser, AppError> {
    let current = repo::get_by_id(pool, id).await?;
    let name = payload.name.unwrap_or(current.name);
    let email = payload.email.map(|e| e.trim().to_lowercase()).unwrap_or(current.email);
    let password_hash = if let Some(pw) = payload.password { hash_password(&pw)? } else { current.password_hash.clone() };
    let rec = sqlx::query_as::<_, User>(
        "UPDATE users SET name=$1, email=$2, password_hash=$3 WHERE id=$4
         RETURNING id, name, email, password_hash, auth_provider, google_sub, is_verified, created_at",
    )
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(rec.into())
}

pub async fn delete(pool: &DbPool, id: Uuid) -> Result<(), AppError> {
    let affected = repo::delete_by_id(pool, id).await?;
    if affected == 0 { return Err(AppError::NotFound("User not found".into())); }
    Ok(())
}
