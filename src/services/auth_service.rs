use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::auth::{create_jwt, hash_password, verify_password};
use crate::config::AppConfig;
use crate::db::DbPool;
use crate::dto::auth::{AuthResponse, GoogleLoginRequest, LoginRequest, RegisterRequest, RequestOtpPayload, ResetPasswordPayload, VerifyOtpPayload};
use crate::errors::AppError;
use crate::mailer;
use crate::models::user::PublicUser;
use crate::repositories::{otp_repo, user_repo, settings_repo};
use crate::services::google;
use rand::Rng;

fn generate_otp_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1_000_000))
}

pub async fn register(pool: &DbPool, cfg: &AppConfig, payload: RegisterRequest) -> Result<AuthResponse<PublicUser>, AppError> {
    let email = payload.email.trim().to_lowercase();
    let hash = hash_password(&payload.password)?;
    let id = Uuid::new_v4();
    let rec = user_repo::insert_local(pool, id, &payload.name, &email, &hash).await
        .map_err(|e| match e {
            AppError::Db(s) if s.contains("unique") => AppError::Conflict("Email already registered".into()),
            other => other,
        })?;
    // Auto-send OTP after register 
    let code = generate_otp_code();
    let expires_at = Utc::now() + Duration::minutes(10);
    if let Err(e) = otp_repo::create(pool, rec.id, &code, "verify", expires_at).await {
        log::warn!("Failed to store OTP for {}: {}", email, e);
    } else if let Err(e) = mailer::send_otp(pool, &email, &code).await {
        log::warn!("SMTP failed for {}: {} (code {})", email, e, code);
    } else {
        log::info!("OTP sent to {} on register", email);
    }
    let token = create_jwt(rec.id, cfg)?;
    Ok(AuthResponse { token, user: rec.into() })
}

pub async fn login(pool: &DbPool, cfg: &AppConfig, payload: LoginRequest) -> Result<AuthResponse<PublicUser>, AppError> {
    let email = payload.email.trim().to_lowercase();
    let user = user_repo::get_by_email(pool, &email).await?.ok_or(AppError::Unauthorized)?;
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }
    if !user.is_verified { return Err(AppError::Forbidden); }
    let token = create_jwt(user.id, cfg)?;
    Ok(AuthResponse { token, user: user.into() })
}

pub async fn me(pool: &DbPool, user_id: Uuid) -> Result<PublicUser, AppError> {
    Ok(user_repo::get_by_id(pool, user_id).await?.into())
}

pub async fn delete_me(pool: &DbPool, user_id: Uuid) -> Result<(), AppError> {
    let affected = user_repo::delete_by_id(pool, user_id).await?;
    if affected == 0 { return Err(AppError::NotFound("User not found".into())); }
    Ok(())
}

pub async fn request_otp(pool: &DbPool, payload: RequestOtpPayload) -> Result<(), AppError> {
    let email = payload.email.trim().to_lowercase();
    let user = user_repo::get_by_email(pool, &email).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
    if let Some(created_at) = otp_repo::last_created_at(pool, user.id, "verify").await? {
        if (Utc::now() - created_at) < Duration::seconds(120) {
            let wait = 120 - (Utc::now() - created_at).num_seconds().max(0);
            return Err(AppError::TooManyRequests(format!("Please wait {}s before requesting another code", wait)));
        }
    }
    let code = generate_otp_code();
    let expires_at = Utc::now() + Duration::minutes(10);
    otp_repo::create(pool, user.id, &code, "verify", expires_at).await?;
    match mailer::send_otp(pool, &email, &code).await {
        Ok(_) => log::info!("OTP sent to {}", email),
        Err(e) => log::warn!("SMTP failed ({}). OTP for {} is {} (dev)", e, email, code),
    }
    Ok(())
}

pub async fn verify_otp(pool: &DbPool, payload: VerifyOtpPayload) -> Result<(), AppError> {
    let email = payload.email.trim().to_lowercase();
    let code = payload.code.trim().to_string();
    let user = user_repo::get_by_email(pool, &email).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
    let otp_id = otp_repo::find_valid(pool, user.id, &code, "verify").await?.ok_or_else(|| AppError::BadRequest("Invalid or expired code".into()))?;
    let mut tx = pool.begin().await.map_err(|e| AppError::Db(e.to_string()))?;
    sqlx::query("UPDATE user_otp_codes SET used_at = now() WHERE id=$1")
        .bind(otp_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Db(e.to_string()))?;
    sqlx::query("UPDATE users SET is_verified=true WHERE id=$1").bind(user.id).execute(&mut *tx).await.map_err(|e| AppError::Db(e.to_string()))?;
    tx.commit().await.map_err(|e| AppError::Db(e.to_string()))?;
    Ok(())
}

pub async fn forgot_password(pool: &DbPool, payload: RequestOtpPayload) -> Result<(), AppError> {
    let email = payload.email.trim().to_lowercase();
    let user = user_repo::get_by_email(pool, &email).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
    if let Some(created_at) = otp_repo::last_created_at(pool, user.id, "reset").await? {
        if (Utc::now() - created_at) < Duration::seconds(120) {
            let wait = 120 - (Utc::now() - created_at).num_seconds().max(0);
            return Err(AppError::TooManyRequests(format!("Please wait {}s before requesting another code", wait)));
        }
    }
    let code = generate_otp_code();
    let expires_at = Utc::now() + Duration::minutes(10);
    otp_repo::create(pool, user.id, &code, "reset", expires_at).await?;
    match mailer::send_otp(pool, &email, &code).await {
        Ok(_) => log::info!("Reset OTP sent to {}", email),
        Err(e) => log::warn!("SMTP failed ({}). Reset OTP for {} is {} (dev)", e, email, code),
    }
    Ok(())
}

pub async fn reset_password(pool: &DbPool, payload: ResetPasswordPayload) -> Result<(), AppError> {
    let email = payload.email.trim().to_lowercase();
    let user = user_repo::get_by_email(pool, &email).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;
    let otp_id = otp_repo::find_valid(pool, user.id, payload.code.trim(), "reset").await?.ok_or_else(|| AppError::BadRequest("Invalid or expired code".into()))?;
    let new_hash = crate::auth::hash_password(&payload.new_password)?;
    let mut tx = pool.begin().await.map_err(|e| AppError::Db(e.to_string()))?;
    sqlx::query("UPDATE user_otp_codes SET used_at = now() WHERE id=$1")
        .bind(otp_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Db(e.to_string()))?;
    sqlx::query("UPDATE users SET password_hash=$1, is_verified=true WHERE id=$2")
        .bind(new_hash)
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Db(e.to_string()))?;
    tx.commit().await.map_err(|e| AppError::Db(e.to_string()))?;
    Ok(())
}

pub async fn google_login(pool: &DbPool, cfg: &AppConfig, req: GoogleLoginRequest) -> Result<AuthResponse<PublicUser>, AppError> {
    // Prefer env, fallback to app_settings
    let client_id = if let Some(id) = cfg.google_client_id.clone() {
        id
    } else {
        settings_repo::get_value(pool, "google_client_id")
            .await?
            .ok_or_else(|| AppError::BadRequest("GOOGLE_CLIENT_ID not configured".into()))?
    };
    let g = google::verify_id_token(&client_id, &req.id_token).await?;
    let email = g.email.to_lowercase();
    let existing = user_repo::get_by_email(pool, &email).await?;
    let user = if let Some(u) = existing {
        let mut u2 = if u.google_sub.is_none() { user_repo::link_google_sub(pool, u.id, &g.sub).await? } else { u };
        if !u2.is_verified && g.email_verified {
            sqlx::query("UPDATE users SET is_verified=true WHERE id=$1")
                .bind(u2.id)
                .execute(pool)
                .await
                .map_err(|e| AppError::Db(e.to_string()))?;
            // reload minimal fields
            u2 = user_repo::get_by_id(pool, u2.id).await?;
        }
        u2
    } else {
        let id = Uuid::new_v4();
        // create unverified then optionally verify if Google asserts email_verified
        let mut created = user_repo::insert_google(pool, id, &g.name, &email, &g.sub).await?;
        if g.email_verified {
            sqlx::query("UPDATE users SET is_verified=true WHERE id=$1")
                .bind(created.id)
                .execute(pool)
                .await
                .map_err(|e| AppError::Db(e.to_string()))?;
            created = user_repo::get_by_id(pool, created.id).await?;
        }
        created
    };
    if !user.is_verified { return Err(AppError::Forbidden); }
    let token = create_jwt(user.id, cfg)?;
    Ok(AuthResponse { token, user: user.into() })
}
