use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Too many requests: {0}")]
    TooManyRequests(String),
    #[error("Database error: {0}")]
    Db(String),
    #[error("Internal server error")]
    Internal,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Db(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized
    }
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: ErrorBody,
}

fn error_code(e: &AppError) -> &'static str {
    match e {
        AppError::Unauthorized => "UNAUTHORIZED",
        AppError::Forbidden => "FORBIDDEN",
        AppError::BadRequest(_) => "BAD_REQUEST",
        AppError::NotFound(_) => "NOT_FOUND",
        AppError::Conflict(_) => "CONFLICT",
        AppError::TooManyRequests(_) => "TOO_MANY_REQUESTS",
        AppError::Db(_) => "DB_ERROR",
        AppError::Internal => "INTERNAL",
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let msg = self.to_string();
        let body = ErrorResponse { success: false, error: ErrorBody { code: error_code(self), message: msg } };
        HttpResponse::build(self.status_code()).json(body)
    }
}
