use crate::{
    models::user::RegisterParams,
    services::app::AppState,
    utils::error::{internal_error, AppError},
};
use actix_web::{web, HttpResponse};

pub async fn register(
    state: web::Data<AppState>,
    params: web::Json<RegisterParams>,
) -> Result<HttpResponse, AppError> {
    let register = state
        .user_service()
        .create_user(params.into_inner())
        .await
        .map_err(internal_error)?;

    Ok(HttpResponse::Ok().json(register))
}

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}
