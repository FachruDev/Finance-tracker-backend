use crate::errors::AppError;
use serde::Deserialize;

#[derive(Debug)]
pub struct GoogleUser {
    pub sub: String,
    pub email: String,
    pub name: String,
}

pub async fn verify_id_token(client_id: &str, id_token: &str) -> Result<GoogleUser, AppError> {
    #[derive(Deserialize)]
    struct TokenInfo {
        aud: String,
        sub: String,
        email: Option<String>,
        name: Option<String>,
    }

    let resp = reqwest::Client::new()
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&[("id_token", id_token.to_string())])
        .send()
        .await
        .map_err(|e| AppError::BadRequest(format!("tokeninfo request failed: {}", e)))?;
    if !resp.status().is_success() {
        return Err(AppError::Unauthorized);
    }
    let info: TokenInfo = resp
        .json()
        .await
        .map_err(|e| AppError::BadRequest(format!("invalid tokeninfo response: {}", e)))?;
    if info.aud != client_id {
        return Err(AppError::Unauthorized);
    }
    Ok(GoogleUser {
        sub: info.sub,
        email: info.email.ok_or_else(|| AppError::BadRequest("Email not present in token".into()))?,
        name: info.name.unwrap_or_else(|| "User".to_string()),
    })
}

