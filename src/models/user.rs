// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    // #[serde(skip_serializing)]
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterParams {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct RegisterResult {
    pub user_id: i64,
    pub username: String,
}
