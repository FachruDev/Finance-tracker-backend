use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{hash_password, AdminUser};
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserReq {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[get("/users")]
pub async fn list_users(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let rows = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, created_at FROM users ORDER BY created_at DESC",
    )
    .fetch_all(pool.get_ref())
    .await?;
    let result: Vec<PublicUser> = rows.into_iter().map(Into::into).collect();
    Ok(HttpResponse::Ok().json(result))
}

#[get("/users/{id}")]
pub async fn get_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let u = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, created_at FROM users WHERE id=$1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let u = u.ok_or_else(|| AppError::NotFound("User not found".into()))?;
    Ok(HttpResponse::Ok().json(PublicUser::from(u)))
}

#[post("/users")]
pub async fn create_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    payload: web::Json<CreateUserReq>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::new_v4();
    let email = payload.email.trim().to_lowercase();
    let hash = hash_password(&payload.password)?;
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
    Ok(HttpResponse::Ok().json(PublicUser::from(rec)))
}

#[put("/users/{id}")]
pub async fn update_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateUserReq>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let mut current = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, created_at FROM users WHERE id=$1",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let mut current = match current.take() {
        Some(u) => u,
        None => return Err(AppError::NotFound("User not found".into())),
    };
    let name = payload.name.clone().unwrap_or(current.name.clone());
    let email = payload
        .email
        .clone()
        .map(|e| e.trim().to_lowercase())
        .unwrap_or(current.email.clone());
    let password_hash = if let Some(pw) = &payload.password {
        crate::auth::hash_password(pw)?
    } else {
        current.password_hash.clone()
    };

    let rec = sqlx::query_as::<_, User>(
        "UPDATE users SET name=$1, email=$2, password_hash=$3 WHERE id=$4
         RETURNING id, name, email, password_hash, created_at",
    )
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(PublicUser::from(rec)))
}

#[delete("/users/{id}")]
pub async fn delete_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let res = sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".into()));
    }
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}

