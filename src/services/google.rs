use crate::errors::AppError;
use serde::Deserialize;

#[derive(Debug)]
pub struct GoogleUser {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub email_verified: bool,
}

pub async fn verify_id_token(client_id: &str, id_token: &str) -> Result<GoogleUser, AppError> {
    #[derive(Deserialize)]
    struct TokenInfo {
        aud: String,
        sub: String,
        email: Option<String>,
        #[serde(default)]
        email_verified: Option<String>,
        name: Option<String>,
    }

    let resp = reqwest::Client::new()
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&[("id_token", id_token.to_string())])
        .send()
        .await
        .map_err(|e| AppError::BadRequest(format!("tokeninfo request failed: {}", e)))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::BadRequest(format!("tokeninfo {}: {}", status, body)));
    }
    let info: TokenInfo = resp
        .json()
        .await
        .map_err(|e| AppError::BadRequest(format!("invalid tokeninfo response: {}", e)))?;
    if info.aud != client_id {
        return Err(AppError::BadRequest(format!("aud mismatch: expected {}, got {}", client_id, info.aud)));
    }
    let email_verified = match info.email_verified.as_deref() {
        Some("true") | Some("True") | Some("1") => true,
        _ => true, 
    };
    Ok(GoogleUser {
        sub: info.sub,
        email: info.email.ok_or_else(|| AppError::BadRequest("Email not present in token".into()))?,
        name: info.name.unwrap_or_else(|| "User".to_string()),
        email_verified,
    })
}

