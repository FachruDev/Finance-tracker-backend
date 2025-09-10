use uuid::Uuid;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::transaction::Transaction;
use crate::dto::transaction::{CreateTransaction, UpdateTransaction, TxnQuery};
use crate::repositories::{transaction_repo as repo, category_repo};

pub async fn list(pool: &DbPool, user_id: Uuid, q: TxnQuery) -> Result<Vec<Transaction>, AppError> {
    repo::list(pool, user_id, q.category_id, q.start_date, q.end_date).await
}

pub async fn create(pool: &DbPool, user_id: Uuid, payload: CreateTransaction) -> Result<Transaction, AppError> {
    // Ensure category belongs to user
    let owner = category_repo::get_by_id_user(pool, payload.category_id, user_id).await?;
    if owner.is_none() { return Err(AppError::BadRequest("Invalid category for user".into())); }
    let id = Uuid::new_v4();
    repo::insert(pool, id, user_id, payload.category_id, &payload.amount, payload.occurred_on, &payload.description).await
}

pub async fn update(pool: &DbPool, user_id: Uuid, id: Uuid, payload: UpdateTransaction) -> Result<Transaction, AppError> {
    let current = repo::get_by_id_user(pool, id, user_id).await?.ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
    let category_id = payload.category_id.unwrap_or(current.category_id);
    if category_id != current.category_id {
        let owner = category_repo::get_by_id_user(pool, category_id, user_id).await?;
        if owner.is_none() { return Err(AppError::BadRequest("Invalid category for user".into())); }
    }
    let amount = payload.amount.unwrap_or(current.amount);
    let occurred_on = payload.occurred_on.unwrap_or(current.occurred_on);
    let description = payload.description.or(current.description);
    repo::update(pool, id, user_id, category_id, &amount, occurred_on, &description).await
}

pub async fn delete(pool: &DbPool, user_id: Uuid, id: Uuid) -> Result<(), AppError> {
    let affected = repo::delete(pool, id, user_id).await?;
    if affected == 0 { return Err(AppError::NotFound("Transaction not found".into())); }
    Ok(())
}

