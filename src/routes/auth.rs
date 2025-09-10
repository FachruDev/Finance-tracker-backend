use actix_web::{get, post, web, HttpResponse};
use crate::config::AppConfig;
use crate::db::DbPool;
use crate::dto::auth::*;
use crate::errors::AppError;
use crate::services::auth_service as svc;

#[post("/auth/register")]
pub async fn register(pool: web::Data<DbPool>, cfg: web::Data<AppConfig>, payload: web::Json<RegisterRequest>) -> Result<HttpResponse, AppError> {
    let res = svc::register(pool.get_ref(), cfg.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/auth/login")]
pub async fn login(pool: web::Data<DbPool>, cfg: web::Data<AppConfig>, payload: web::Json<LoginRequest>) -> Result<HttpResponse, AppError> {
    let res = svc::login(pool.get_ref(), cfg.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/me")]
pub async fn me(pool: web::Data<DbPool>, user: crate::auth::AuthUser) -> Result<HttpResponse, AppError> {
    let res = svc::me(pool.get_ref(), user.0).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::delete("/me")]
pub async fn delete_me(pool: web::Data<DbPool>, user: crate::auth::AuthUser) -> Result<HttpResponse, AppError> {
    svc::delete_me(pool.get_ref(), user.0).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/auth/request-otp")]
pub async fn request_otp(pool: web::Data<DbPool>, payload: web::Json<RequestOtpPayload>) -> Result<HttpResponse, AppError> {
    svc::request_otp(pool.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"ok": true})))
}

#[post("/auth/verify-otp")]
pub async fn verify_otp(pool: web::Data<DbPool>, payload: web::Json<VerifyOtpPayload>) -> Result<HttpResponse, AppError> {
    svc::verify_otp(pool.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"verified": true})))
}

#[post("/auth/forgot-password")]
pub async fn forgot_password(pool: web::Data<DbPool>, payload: web::Json<RequestOtpPayload>) -> Result<HttpResponse, AppError> {
    svc::forgot_password(pool.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"ok": true})))
}

#[post("/auth/reset-password")]
pub async fn reset_password(pool: web::Data<DbPool>, payload: web::Json<ResetPasswordPayload>) -> Result<HttpResponse, AppError> {
    svc::reset_password(pool.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"reset": true})))
}

#[post("/auth/logout")]
pub async fn logout(_user: crate::auth::AuthUser) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({"ok": true})))
}

#[post("/auth/google")]
pub async fn google_login(pool: web::Data<DbPool>, cfg: web::Data<AppConfig>, payload: web::Json<GoogleLoginRequest>) -> Result<HttpResponse, AppError> {
    let res = svc::google_login(pool.get_ref(), cfg.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(login)
        .service(me)
        .service(delete_me)
        .service(request_otp)
        .service(verify_otp)
        .service(google_login)
        .service(forgot_password)
        .service(reset_password)
        .service(logout);
}
