use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse<T> {
    pub token: String,
    pub user: T,
}

#[derive(Debug, Deserialize)]
pub struct RequestOtpPayload {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOtpPayload {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleLoginRequest {
    pub id_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordPayload {
    pub email: String,
    pub code: String,
    pub new_password: String,
}

