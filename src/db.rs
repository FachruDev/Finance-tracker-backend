use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub type DbPool = PgPool;

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    let allow_mismatch = env::var("MIGRATION_ALLOW_MISMATCH")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("1") || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(()) => Ok(()),
        Err(e @ sqlx::migrate::MigrateError::VersionMismatch(_)) if allow_mismatch => {
            log::warn!(
                "sqlx migration version mismatch detected; proceeding due to MIGRATION_ALLOW_MISMATCH=true. Error: {}",
                e
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}
