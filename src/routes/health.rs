use actix_web::{get, HttpResponse, Responder, web};

#[get("/healthz")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status":"ok"}))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}

