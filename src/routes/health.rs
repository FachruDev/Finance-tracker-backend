use actix_web::{get, Responder, web};
use crate::response as resp;

#[get("/healthz")]
async fn health() -> impl Responder {
    resp::message("ok")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}
