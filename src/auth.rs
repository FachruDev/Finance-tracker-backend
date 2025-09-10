use crate::config::AppConfig;
use crate::errors::AppError;
use actix_web::{dev::Payload, http::header, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use password_hash::{PasswordHash, PasswordVerifier, SaltString};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_jwt(user_id: Uuid, cfg: &AppConfig) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::hours(cfg.jwt_exp_hours as i64);
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: &str, cfg: &AppConfig) -> Result<Claims, AppError> {
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

pub fn hash_password(plain: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2::Argon2::default();
    let hash = argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|_| AppError::Internal)?
        .to_string();
    Ok(hash)
}

pub fn verify_password(plain: &str, hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(hash).map_err(|_| AppError::Internal)?;
    Ok(argon2::Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok())
}

#[derive(Debug, Clone, Copy)]
pub struct AuthUser(pub Uuid);

impl FromRequest for AuthUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = match req.app_data::<actix_web::web::Data<AppConfig>>() {
            Some(c) => c.clone(),
            None => return ready(Err(AppError::Internal)),
        };

        let auth = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_string();

        if let Some(token) = auth.strip_prefix("Bearer ") {
            match verify_jwt(token, &cfg) {
                Ok(claims) => ready(Ok(AuthUser(claims.sub))),
                Err(_) => ready(Err(AppError::Unauthorized)),
            }
        } else {
            ready(Err(AppError::Unauthorized))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AdminUser(pub Uuid);

impl FromRequest for AdminUser {
    type Error = AppError;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = req.app_data::<actix_web::web::Data<AppConfig>>().cloned();
        let pool = req
            .app_data::<actix_web::web::Data<crate::db::DbPool>>()
            .cloned();
        let auth = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        Box::pin(async move {
            let cfg = cfg.ok_or(AppError::Internal)?;
            let pool = pool.ok_or(AppError::Internal)?;
            let auth = auth.ok_or(AppError::Unauthorized)?;
            let token = auth.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;
            let claims = verify_jwt(token, &cfg)?;
            let count: (i64,) = sqlx::query_as(
                "SELECT COUNT(1) FROM admins WHERE id = $1",
            )
            .bind(claims.sub)
            .fetch_one(pool.get_ref())
            .await
            .map_err(|_| AppError::Unauthorized)?;
            if count.0 == 0 {
                return Err(AppError::Forbidden);
            }
            Ok(AdminUser(claims.sub))
        })
    }
}
