use actix_web::web;

pub mod auth;
pub mod users;
pub mod settings;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/admin")
        .configure(auth::config)
        .configure(users::config)
        .configure(settings::config));
}

