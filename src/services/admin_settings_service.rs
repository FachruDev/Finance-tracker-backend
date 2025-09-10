use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::setting::Setting;
use crate::repositories::settings_repo as repo;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct UpdateSetting { pub value: String }

pub async fn list(pool: &DbPool) -> Result<Vec<Setting>, AppError> { repo::list_all(pool).await }

pub async fn upsert(pool: &DbPool, key: String, value: String, admin_id: Uuid) -> Result<Setting, AppError> {
    repo::upsert(pool, &key, &value, admin_id).await
}

