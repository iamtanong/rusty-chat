use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

        AppConfig {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .expect("SERVER_PORT must be a number"),
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                max_connections: std::env::var("DATABASE_MAX_CONNECTION")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .expect("DATABASE_MAX_CONNECTION must be a number"),
            },
        }
    }
}
