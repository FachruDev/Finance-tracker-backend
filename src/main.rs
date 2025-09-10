mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config::AppConfig;
use db::DbPool;
use std::net::SocketAddr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let cfg = AppConfig::from_env();
    let addr: SocketAddr = format!("{}:{}", cfg.app_host, cfg.app_port)
        .parse()
        .expect("invalid host/port");

    let pool: DbPool = db::init_pool(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    // Run migrations at startup
    db::run_migrations(&pool).await.expect("migrations failed");

    log::info!("Starting server at http://{}", addr);

    HttpServer::new(move || {
        // CORS: by default allow configured origins or *
        let mut cors = Cors::default()
            .allow_any_header()
            .allow_any_method();
        if let Some(origins) = &cfg.cors_allowed_origins {
            if origins == "*" {
                cors = cors.allow_any_origin();
            } else {
                for origin in origins.split(',') {
                    cors = cors.allowed_origin(origin.trim());
                }
            }
        } else {
            cors = cors.allow_any_origin();
        }

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(addr)?
    .workers(2)
    .run()
    .await
}
