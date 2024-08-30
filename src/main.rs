use rusty_chat::{config::AppConfig, startup::run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::new();

    let listener = TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))?;
    run(listener, config).await
}
