use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::auth::AdminUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::services::admin_user_service as svc;

#[get("/users")]
pub async fn list_users(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let rows = svc::list(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[get("/users/{id}")]
pub async fn get_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let u = svc::get(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(u))
}

#[post("/users")]
pub async fn create_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    payload: web::Json<svc::CreateUserReq>,
) -> Result<HttpResponse, AppError> {
    let rec = svc::create(pool.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(rec))
}

#[put("/users/{id}")]
pub async fn update_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    payload: web::Json<svc::UpdateUserReq>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let rec = svc::update(pool.get_ref(), id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(rec))
}

#[delete("/users/{id}")]
pub async fn delete_user(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    svc::delete(pool.get_ref(), id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user);
}
