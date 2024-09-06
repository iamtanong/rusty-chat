use crate::{models::user::RegisterResult, utils::error::AppError};

use sqlx::{self, PgPool};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<RegisterResult, AppError> {
        let user = match sqlx::query_as!(
            RegisterResult,
            r#"
            INSERT INTO "users" (username, password)
            VALUES ($1, $2)
            RETURNING id AS user_id, username;
        "#,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(result) => result,
            Err(err) => {
                eprintln!("{:?}", err);
                return Err(AppError::InternalServerError);
            }
        };

        Ok(user)
    }
}
