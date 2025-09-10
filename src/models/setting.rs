use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

// UpdateSetting DTO moved to services/admin_settings_service.rs
