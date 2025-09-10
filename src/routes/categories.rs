use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::category::{Category, CreateCategory, UpdateCategory};

fn validate_kind(kind: &str) -> bool {
    matches!(kind, "income" | "expense")
}

#[get("/categories")]
pub async fn list_categories(
    pool: web::Data<DbPool>,
    user: AuthUser,
) -> Result<HttpResponse, AppError> {
    let rows = sqlx::query_as::<_, Category>(
        "SELECT id, user_id, name, kind, color, created_at FROM categories WHERE user_id=$1 ORDER BY name",
    )
    .bind(user.0)
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[post("/categories")]
pub async fn create_category(
    pool: web::Data<DbPool>,
    user: AuthUser,
    payload: web::Json<CreateCategory>,
) -> Result<HttpResponse, AppError> {
    if !validate_kind(&payload.kind) {
        return Err(AppError::BadRequest("kind must be 'income' or 'expense'".into()));
    }
    let id = Uuid::new_v4();
    let color = payload.color.clone().unwrap_or_else(|| "#888888".to_string());

    let row = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (id, user_id, name, kind, color)
         VALUES ($1,$2,$3,$4,$5)
         RETURNING id, user_id, name, kind, color, created_at",
    )
    .bind(id)
    .bind(user.0)
    .bind(&payload.name)
    .bind(&payload.kind)
    .bind(&color)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db) = &e {
            if db.is_unique_violation() {
                return AppError::Conflict("Category already exists".into());
            }
        }
        e.into()
    })?;
    Ok(HttpResponse::Ok().json(row))
}

#[put("/categories/{id}")]
pub async fn update_category(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateCategory>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    // Fetch current
    let mut current = sqlx::query_as::<_, Category>(
        "SELECT id, user_id, name, kind, color, created_at FROM categories WHERE id=$1 AND user_id=$2",
    )
    .bind(id)
    .bind(user.0)
    .fetch_optional(pool.get_ref())
    .await?;

    let current = match current.take() {
        Some(c) => c,
        None => return Err(AppError::NotFound("Category not found".into())),
    };

    if let Some(kind) = &payload.kind {
        if !validate_kind(kind) {
            return Err(AppError::BadRequest("kind must be 'income' or 'expense'".into()));
        }
    }

    let name = payload.name.clone().unwrap_or(current.name.clone());
    let kind = payload.kind.clone().unwrap_or(current.kind.clone());
    let color = payload.color.clone().unwrap_or(current.color.clone());

    let updated = sqlx::query_as::<_, Category>(
        "UPDATE categories SET name=$1, kind=$2, color=$3 WHERE id=$4 AND user_id=$5
         RETURNING id, user_id, name, kind, color, created_at",
    )
    .bind(name)
    .bind(kind)
    .bind(color)
    .bind(id)
    .bind(user.0)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/categories/{id}")]
pub async fn delete_category(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let res = sqlx::query(
        "DELETE FROM categories WHERE id=$1 AND user_id=$2",
    )
    .bind(id)
    .bind(user.0)
    .execute(pool.get_ref())
    .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::NotFound("Category not found".into()));
    }
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_categories)
        .service(create_category)
        .service(update_category)
        .service(delete_category);
}
