use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_exp_hours: u64,
    pub cors_allowed_origins: Option<String>,
    pub google_client_id: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let app_port = env::var("APP_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8080);
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set (e.g., postgres://user:pass@localhost/db)");
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            // For dev, generate a weak default; in prod must be set
            log::warn!("JWT_SECRET not set; using insecure default for development.");
            "dev-secret-change-me".to_string()
        });
        let jwt_exp_hours = env::var("JWT_EXP_HOURS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(24 * 7);
        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS").ok();
        let google_client_id = env::var("GOOGLE_CLIENT_ID").ok();

        Self {
            app_host,
            app_port,
            database_url,
            jwt_secret,
            jwt_exp_hours,
            cors_allowed_origins,
            google_client_id,
        }
    }
}
