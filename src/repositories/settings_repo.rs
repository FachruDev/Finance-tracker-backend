use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::setting::Setting;
use uuid::Uuid;

pub async fn list_all(pool: &DbPool) -> Result<Vec<Setting>, AppError> {
    let rows = sqlx::query_as::<_, Setting>(
        "SELECT key, value, updated_by, updated_at FROM app_settings ORDER BY key",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn upsert(pool: &DbPool, key: &str, value: &str, admin_id: Uuid) -> Result<Setting, AppError> {
    let row = sqlx::query_as::<_, Setting>(
        r#"INSERT INTO app_settings (key, value, updated_by, updated_at)
           VALUES ($1,$2,$3, now())
           ON CONFLICT (key) DO UPDATE SET value=EXCLUDED.value, updated_by=EXCLUDED.updated_by, updated_at=now()
           RETURNING key, value, updated_by, updated_at"#,
    )
    .bind(key)
    .bind(value)
    .bind(admin_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

