use sqlx::PgPool;

use crate::{
    db::repositories::user::UserRepository,
    models::user::{RegisterParams, RegisterResult},
    utils::error::AppError,
};

#[derive(Debug, Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(db_pool: PgPool) -> Self {
        let user_repo = UserRepository::new(db_pool);
        Self { user_repo }
    }

    pub async fn create_user(&self, params: RegisterParams) -> Result<RegisterResult, AppError> {
        let register = self
            .user_repo
            .create_user(&params.username, &params.password)
            .await;

        register
    }
}
