use actix_web::{get, put, web, HttpResponse};
use serde::Deserialize;

use crate::auth::AdminUser;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::setting::{Setting, UpdateSetting};

#[get("/settings")]
pub async fn list_settings(
    _admin: AdminUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let rows = sqlx::query_as::<_, Setting>(
        "SELECT key, value, updated_by, updated_at FROM app_settings ORDER BY key",
    )
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[put("/settings/{key}")]
pub async fn upsert_setting(
    admin: AdminUser,
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    payload: web::Json<UpdateSetting>,
) -> Result<HttpResponse, AppError> {
    let key = path.into_inner();
    let value = payload.value.clone();
    let row = sqlx::query_as::<_, Setting>(
        r#"INSERT INTO app_settings (key, value, updated_by, updated_at)
           VALUES ($1,$2,$3, now())
           ON CONFLICT (key) DO UPDATE SET value=EXCLUDED.value, updated_by=EXCLUDED.updated_by, updated_at=now()
           RETURNING key, value, updated_by, updated_at"#,
    )
    .bind(&key)
    .bind(&value)
    .bind(admin.0)
    .fetch_one(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(row))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_settings).service(upsert_setting);
}

