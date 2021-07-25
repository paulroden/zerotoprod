//! src/main.rs
use zerotoprod::startup::run;
use zerotoprod::configuration::get_configuration;
use zerotoprod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use sqlx::PgPool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zerotoprod".into(), "info".into());
    init_subscriber(subscriber);
    
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
