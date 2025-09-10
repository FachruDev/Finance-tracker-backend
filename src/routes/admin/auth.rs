use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{create_jwt, hash_password, verify_password, AdminUser};
use crate::config::AppConfig;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::admin::{Admin, PublicAdmin};

#[derive(Debug, Deserialize)]
pub struct AdminRegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AdminAuthResponse {
    pub token: String,
    pub admin: PublicAdmin,
}

// Bootstrap rule: if no admin exists, allow first registration without auth; otherwise require admin auth
#[post("/auth/register")]
pub async fn register_admin(
    pool: web::Data<DbPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<AdminRegisterRequest>,
    maybe_admin: Option<AdminUser>,
) -> Result<HttpResponse, AppError> {
    let (exists,): (i64,) = sqlx::query_as("SELECT COUNT(1) FROM admins")
        .fetch_one(pool.get_ref())
        .await?;
    if exists > 0 && maybe_admin.is_none() {
        return Err(AppError::Forbidden);
    }

    let email = payload.email.trim().to_lowercase();
    let hash = hash_password(&payload.password)?;
    let id = Uuid::new_v4();

    let rec = sqlx::query_as::<_, Admin>(
        "INSERT INTO admins (id, name, email, password_hash) VALUES ($1,$2,$3,$4)
         RETURNING id, name, email, password_hash, created_at",
    )
    .bind(id)
    .bind(&payload.name)
    .bind(&email)
    .bind(&hash)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db) = &e {
            if db.is_unique_violation() {
                return AppError::Conflict("Admin email already exists".into());
            }
        }
        e.into()
    })?;

    let token = create_jwt(rec.id, &cfg)?;
    Ok(HttpResponse::Ok().json(AdminAuthResponse { token, admin: rec.into() }))
}

#[post("/auth/login")]
pub async fn login_admin(
    pool: web::Data<DbPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<AdminLoginRequest>,
) -> Result<HttpResponse, AppError> {
    let email = payload.email.trim().to_lowercase();
    let admin = sqlx::query_as::<_, Admin>(
        "SELECT id, name, email, password_hash, created_at FROM admins WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await?;
    let admin = match admin { Some(a) => a, None => return Err(AppError::Unauthorized) };
    if !verify_password(&payload.password, &admin.password_hash)? {
        return Err(AppError::Unauthorized);
    }
    let token = create_jwt(admin.id, &cfg)?;
    Ok(HttpResponse::Ok().json(AdminAuthResponse { token, admin: admin.into() }))
}

#[get("/me")]
pub async fn me_admin(
    pool: web::Data<DbPool>,
    admin: AdminUser,
) -> Result<HttpResponse, AppError> {
    let a = sqlx::query_as::<_, Admin>(
        "SELECT id, name, email, password_hash, created_at FROM admins WHERE id = $1",
    )
    .bind(admin.0)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(PublicAdmin::from(a)))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_admin).service(login_admin).service(me_admin);
}

