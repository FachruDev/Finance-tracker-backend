use actix_web::{post, get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{create_jwt, hash_password, verify_password, AuthUser};
use crate::config::AppConfig;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: PublicUser,
}

#[post("/auth/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let email = payload.email.trim().to_lowercase();
    let hash = hash_password(&payload.password)?;
    let id = Uuid::new_v4();

    let rec = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email, password_hash) VALUES ($1,$2,$3,$4)
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
                return AppError::Conflict("Email already registered".into());
            }
        }
        e.into()
    })?;

    let token = create_jwt(rec.id, &cfg)?;
    Ok(HttpResponse::Ok().json(AuthResponse { token, user: rec.into() }))
}

#[post("/auth/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let email = payload.email.trim().to_lowercase();
    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, created_at FROM users WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await?;

    let user = match user {
        Some(u) => u,
        None => return Err(AppError::Unauthorized),
    };

    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = create_jwt(user.id, &cfg)?;
    Ok(HttpResponse::Ok().json(AuthResponse { token, user: user.into() }))
}

#[get("/me")]
pub async fn me(
    pool: web::Data<DbPool>,
    user: crate::auth::AuthUser,
) -> Result<HttpResponse, AppError> {
    let u = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, created_at FROM users WHERE id = $1",
    )
    .bind(user.0)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(PublicUser::from(u)))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login).service(me);
}

