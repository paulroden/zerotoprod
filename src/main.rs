//! src/main.rs
use zerotoprod::startup::run;
use zerotoprod::configuration::get_configuration;
use zerotoprod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zerotoprod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    
    // Try to read configuration and panic if this fails
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // Take TCP port number from configuration
    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port,
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
