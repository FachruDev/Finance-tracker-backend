use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Admin {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicAdmin {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<Admin> for PublicAdmin {
    fn from(a: Admin) -> Self {
        Self {
            id: a.id,
            name: a.name,
            email: a.email,
            created_at: a.created_at,
        }
    }
}

