use actix_web::web;

mod health;
pub mod auth;
pub mod categories;
pub mod transactions;
pub mod summary;
pub mod admin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .configure(health::config)
        .configure(auth::config)
        .configure(categories::config)
        .configure(transactions::config)
        .configure(summary::config)
        .configure(admin::config));
}
