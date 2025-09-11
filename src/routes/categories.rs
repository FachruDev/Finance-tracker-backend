use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::dto::category::{CreateCategory, UpdateCategory};
use crate::services::category_service as svc;
use crate::response as resp;


#[get("/categories")]
pub async fn list_categories(
    pool: web::Data<DbPool>,
    user: AuthUser,
) -> Result<HttpResponse, AppError> {
    let rows = svc::list(pool.get_ref(), user.0).await?;
    Ok(resp::ok(rows))
}

#[post("/categories")]
pub async fn create_category(
    pool: web::Data<DbPool>,
    user: AuthUser,
    payload: web::Json<CreateCategory>,
) -> Result<HttpResponse, AppError> {
    let row = svc::create(pool.get_ref(), user.0, payload.into_inner()).await?;
    Ok(resp::created(row))
}

#[put("/categories/{id}")]
pub async fn update_category(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateCategory>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let row = svc::update(pool.get_ref(), user.0, id, payload.into_inner()).await?;
    Ok(resp::ok(row))
}

#[delete("/categories/{id}")]
pub async fn delete_category(
    pool: web::Data<DbPool>,
    user: AuthUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    svc::delete(pool.get_ref(), user.0, id).await?;
    Ok(resp::message("Category deleted"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_categories)
        .service(create_category)
        .service(update_category)
        .service(delete_category);
}
