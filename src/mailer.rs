use crate::db::DbPool;
use crate::errors::AppError;
use lettre::message::{header, Mailbox, Message};
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::transport::smtp::client::{Tls, TlsParameters};

// Reads SMTP settings from app_settings and sends an email.
// Required keys: smtp_host, smtp_port, smtp_username, smtp_password, smtp_from, smtp_tls (optional: "true"/"false", default true)
async fn get_setting(pool: &DbPool, key: &str) -> Result<Option<String>, AppError> {
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM app_settings WHERE key=$1")
        .bind(key)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Db(e.to_string()))?;
    Ok(row.map(|r| r.0))
}

pub async fn send_email(
    pool: &DbPool,
    to_email: &str,
    subject: &str,
    body_text: &str,
) -> Result<(), AppError> {
    let host = get_setting(pool, "smtp_host").await?.ok_or_else(|| AppError::BadRequest("SMTP not configured".into()))?;
    let port: u16 = get_setting(pool, "smtp_port").await?
        .ok_or_else(|| AppError::BadRequest("SMTP not configured".into()))?
        .parse()
        .map_err(|_| AppError::BadRequest("Invalid smtp_port".into()))?;
    let username = get_setting(pool, "smtp_username").await?.unwrap_or_default();
    let password = get_setting(pool, "smtp_password").await?.unwrap_or_default();
    let from = get_setting(pool, "smtp_from").await?.ok_or_else(|| AppError::BadRequest("smtp_from missing".into()))?;
    let use_tls = get_setting(pool, "smtp_tls").await?.unwrap_or_else(|| "true".into()) == "true";

    let creds = Credentials::new(username.clone(), password.clone());
    let mailer = if use_tls {
        // Use STARTTLS by default (port 587). If port 465, use implicit TLS wrapper.
        if port == 465 {
            let tls = TlsParameters::builder(host.clone())
                .build()
                .map_err(|e| AppError::BadRequest(format!("SMTP TLS params error: {}", e)))?;
            AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
                .map_err(|e| AppError::BadRequest(format!("SMTP relay error: {}", e)))?
                .tls(Tls::Wrapper(tls))
                .port(port)
                .credentials(creds)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)
                .map_err(|e| AppError::BadRequest(format!("SMTP relay error: {}", e)))?
                .port(port)
                .credentials(creds)
                .build()
        }
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&host)
            .port(port)
            .credentials(creds)
            .build()
    };

    let email = Message::builder()
        .from(from.parse::<Mailbox>().map_err(|_| AppError::BadRequest("Invalid smtp_from".into()))?)
        .to(to_email.parse::<Mailbox>().map_err(|_| AppError::BadRequest("Invalid recipient".into()))?)
        .subject(subject)
        .header(header::ContentType::TEXT_PLAIN)
        .body(body_text.to_string())
        .map_err(|e| AppError::BadRequest(format!("Email build error: {}", e)))?;

    mailer
        .send(email)
        .await
        .map_err(|e| AppError::BadRequest(format!("SMTP send error: {}", e)))?;
    Ok(())
}

pub async fn send_otp(
    pool: &DbPool,
    to_email: &str,
    code: &str,
) -> Result<(), AppError> {
    let subject = "Your verification code";
    let body = format!("Your OTP code is: {}\nThis code expires in 10 minutes.", code);
    send_email(pool, to_email, subject, &body).await
}
