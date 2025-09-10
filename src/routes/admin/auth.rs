use actix_web::{get, post, web, HttpResponse};
use crate::auth::AdminUser;
use crate::config::AppConfig;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::services::admin_auth_service as svc;

// Bootstrap rule: if no admin exists, allow first registration without auth; otherwise require admin auth
#[post("/auth/register")]
pub async fn register_admin(
    pool: web::Data<DbPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<svc::AdminRegisterRequest>,
    maybe_admin: Option<AdminUser>,
) -> Result<HttpResponse, AppError> {
    let res = svc::register(pool.get_ref(), cfg.get_ref(), payload.into_inner(), maybe_admin.is_some()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/auth/login")]
pub async fn login_admin(
    pool: web::Data<DbPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<svc::AdminLoginRequest>,
) -> Result<HttpResponse, AppError> {
    let res = svc::login(pool.get_ref(), cfg.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/me")]
pub async fn me_admin(
    pool: web::Data<DbPool>,
    admin: AdminUser,
) -> Result<HttpResponse, AppError> {
    let a = svc::me(pool.get_ref(), admin.0).await?;
    Ok(HttpResponse::Ok().json(a))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_admin).service(login_admin).service(me_admin);
}
