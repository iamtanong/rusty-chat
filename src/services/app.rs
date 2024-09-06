use sqlx::PgPool;

use super::user::UserService;

#[derive(Clone, Debug)]
pub struct AppState {
    db_pool: PgPool,
    // services
    user_service: UserService,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        let user_service = UserService::new(db_pool.clone());

        Self {
            db_pool,
            user_service,
        }
    }

    pub fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }

    pub fn user_service(&self) -> &UserService {
        &self.user_service
    }
}
