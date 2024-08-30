use crate::{api, config::AppConfig};
use actix_web::{rt::signal, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

pub async fn run(listener: TcpListener, config: AppConfig) -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
        .expect("Failed to create pool");

    // To check migrations: sqlxmigrate info
    // To run migrations: sqlx migrate run

    // Create app state
    let app_state = web::Data::new(AppState { db: pool.clone() });

    // Start HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(api::routes::route_config)
    })
    .listen(listener)?
    .run();
    println!("Server running at https://localhost:8080");

    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            println!(" Ctrl-C Recieve, Shutting down");
        },
    };

    Ok(())
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: sqlx::PgPool,
}
