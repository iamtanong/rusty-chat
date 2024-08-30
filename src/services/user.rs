use actix_web::{web::Data, Error};

use crate::{
    db::repositories::user as db,
    models::user::{RegisterParams, RegisterResult},
    startup::AppState,
};

pub async fn register(
    app_state: &Data<AppState>,
    params: RegisterParams,
) -> Result<RegisterResult, Error> {
    let register: RegisterResult =
        db::create_user(&app_state.db, &params.username, &params.password)
            .await
            .unwrap();

    Ok(register)
}
