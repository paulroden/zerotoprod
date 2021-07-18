//! src/main.rs
use zerotoprod::startup::run;
use zerotoprod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;
use env_logger::Env;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Instantiate logging middleware
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Try to read configuration and panic if this fails
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // Take TCP port number from configuration
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
