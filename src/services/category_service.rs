use uuid::Uuid;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::category::Category;
use crate::dto::category::{CreateCategory, UpdateCategory};
use crate::repositories::category_repo as repo;

fn validate_kind(kind: &str) -> bool { matches!(kind, "income" | "expense") }

pub async fn list(pool: &DbPool, user_id: Uuid) -> Result<Vec<Category>, AppError> {
    repo::list_by_user(pool, user_id).await
}

pub async fn create(pool: &DbPool, user_id: Uuid, payload: CreateCategory) -> Result<Category, AppError> {
    if !validate_kind(&payload.kind) { return Err(AppError::BadRequest("kind must be 'income' or 'expense'".into())); }
    let id = Uuid::new_v4();
    let color = payload.color.unwrap_or_else(|| "#888888".into());
    let inserted = repo::insert(pool, id, user_id, &payload.name, &payload.kind, &color).await
        .map_err(|e| match e {
            AppError::Db(s) if s.contains("unique") => AppError::Conflict("Category already exists".into()),
            other => other,
        })?;
    Ok(inserted)
}

pub async fn update(pool: &DbPool, user_id: Uuid, id: Uuid, payload: UpdateCategory) -> Result<Category, AppError> {
    let current = repo::get_by_id_user(pool, id, user_id).await?.ok_or_else(|| AppError::NotFound("Category not found".into()))?;
    if let Some(kind) = &payload.kind { if !validate_kind(kind) { return Err(AppError::BadRequest("kind must be 'income' or 'expense'".into())); } }
    let name = payload.name.unwrap_or(current.name);
    let kind = payload.kind.unwrap_or(current.kind);
    let color = payload.color.unwrap_or(current.color);
    let updated = repo::update(pool, id, user_id, &name, &kind, &color).await?;
    Ok(updated)
}

pub async fn delete(pool: &DbPool, user_id: Uuid, id: Uuid) -> Result<(), AppError> {
    let affected = repo::delete(pool, id, user_id).await?;
    if affected == 0 { return Err(AppError::NotFound("Category not found".into())); }
    Ok(())
}

