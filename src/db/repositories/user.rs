use crate::{models::user::RegisterResult, utils::error::AppError};

use sqlx::{self, Pool, Postgres};

pub async fn create_user(
    pool: &Pool<Postgres>,
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
    .fetch_one(pool)
    .await
    {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(AppError::InternalServerError);
        }
    };

    println!("User: {:?}", user);

    Ok(RegisterResult {
        user_id: user.user_id,
        username: user.username,
    })
}
