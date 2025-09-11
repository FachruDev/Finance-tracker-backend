use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ApiMessage {
    pub success: bool,
    pub message: String,
}

pub fn ok<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse { success: true, data, message: None })
}

pub fn created<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Created().json(ApiResponse { success: true, data, message: None })
}

pub fn message(msg: impl Into<String>) -> HttpResponse {
    HttpResponse::Ok().json(ApiMessage { success: true, message: msg.into() })
}

