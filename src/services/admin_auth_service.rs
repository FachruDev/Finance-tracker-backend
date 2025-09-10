use uuid::Uuid;
use crate::auth::{hash_password, verify_password, create_jwt};
use crate::config::AppConfig;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::admin::PublicAdmin;
use crate::repositories::admin_repo as repo;

#[derive(Debug, serde::Deserialize)]
pub struct AdminRegisterRequest { pub name: String, pub email: String, pub password: String }

#[derive(Debug, serde::Deserialize)]
pub struct AdminLoginRequest { pub email: String, pub password: String }

#[derive(Debug, serde::Serialize)]
pub struct AdminAuthResponse { pub token: String, pub admin: PublicAdmin }

pub async fn register(pool: &DbPool, cfg: &AppConfig, payload: AdminRegisterRequest, bootstrap_ok: bool) -> Result<AdminAuthResponse, AppError> {
    let exists = repo::count_admins(pool).await? > 0;
    if exists && !bootstrap_ok { return Err(AppError::Forbidden); }
    let email = payload.email.trim().to_lowercase();
    let hash = hash_password(&payload.password)?;
    let id = Uuid::new_v4();
    let rec = repo::insert(pool, id, &payload.name, &email, &hash).await
        .map_err(|e| match e { AppError::Db(s) if s.contains("unique") => AppError::Conflict("Admin email already exists".into()), other => other })?;
    let token = create_jwt(rec.id, cfg)?;
    Ok(AdminAuthResponse { token, admin: rec.into() })
}

pub async fn login(pool: &DbPool, cfg: &AppConfig, payload: AdminLoginRequest) -> Result<AdminAuthResponse, AppError> {
    let email = payload.email.trim().to_lowercase();
    let admin = repo::get_by_email(pool, &email).await?.ok_or(AppError::Unauthorized)?;
    if !verify_password(&payload.password, &admin.password_hash)? { return Err(AppError::Unauthorized); }
    let token = create_jwt(admin.id, cfg)?;
    Ok(AdminAuthResponse { token, admin: admin.into() })
}

pub async fn me(pool: &DbPool, admin_id: Uuid) -> Result<PublicAdmin, AppError> {
    Ok(repo::get_by_id(pool, admin_id).await?.into())
}
