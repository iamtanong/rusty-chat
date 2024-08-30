use crate::{models::user::RegisterParams, services, startup::AppState};
use actix_web::{web, HttpResponse};

pub async fn register(
    data: web::Data<AppState>,
    params: web::Json<RegisterParams>,
) -> HttpResponse {
    let register = services::user::register(&data, params.into_inner())
        .await
        .unwrap();

    HttpResponse::Ok().json(register)
}

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}
