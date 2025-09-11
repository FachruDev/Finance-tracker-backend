use actix_web::{get, put, web, HttpResponse};

use crate::auth::AdminUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::services::admin_settings_service as svc;
use crate::response as resp;

#[get("/settings")]
pub async fn list_settings(_admin: AdminUser, pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let rows = svc::list(pool.get_ref()).await?;
    Ok(resp::ok(rows))
}

#[put("/settings/{key}")]
pub async fn upsert_setting(admin: AdminUser, pool: web::Data<DbPool>, path: web::Path<String>, payload: web::Json<svc::UpdateSetting>) -> Result<HttpResponse, AppError> {
    let key = path.into_inner();
    let row = svc::upsert(pool.get_ref(), key, payload.value.clone(), admin.0).await?;
    Ok(resp::ok(row))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_settings).service(upsert_setting);
}
