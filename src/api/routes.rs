use actix_web::web;

use super::handler;

// Configure the HTTP API
pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/user")
                .route("/register", web::post().to(handler::user::register))
                .route("/login", web::post().to(handler::user::login)),
        ),
    );
}
