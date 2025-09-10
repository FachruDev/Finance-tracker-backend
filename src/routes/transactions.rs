use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::dto::transaction::{CreateTransaction, TxnQuery, UpdateTransaction};
use crate::services::transaction_service as svc;

#[get("/transactions")]
pub async fn list_transactions(
    pool: web::Data<DbPool>,
    user: AuthUser,
    query: web::Query<TxnQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let rows = svc::list(pool.get_ref(), user.0, q).await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[post("/transactions")]
pub async fn create_transaction(
    pool: web::Data<DbPool>,
    user: AuthUser,
    payload: web::Json<CreateTransaction>,
) -> Result<HttpResponse, AppError> {
    let rec = svc::create(pool.get_ref(), user.0, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(rec))
}

#[put("/transactions/{id}")]
pub async fn update_transaction(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateTransaction>,
) -> Result<HttpResponse, AppError> {
    let id_val = path.into_inner();
    let updated = svc::update(pool.get_ref(), user.0, id_val, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/transactions/{id}")]
pub async fn delete_transaction(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id_val = path.into_inner();
    svc::delete(pool.get_ref(), user.0, id_val).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_transactions)
        .service(create_transaction)
        .service(update_transaction)
        .service(delete_transaction);
}
