//! src/main.rs
use zerotoprod::startup::run;
use zerotoprod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // Fall back to having all tracing spans at INFO level or abive
    // if the RUST_LOG environment has not been set.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zerotoprod".into(),
        // Output the spans to stdout.
        std::io::stdout
    );
    // set-up tracing subscriber using settings above and
    // instantiate with `set_global_default`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set tracing subscriber.");
    
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
