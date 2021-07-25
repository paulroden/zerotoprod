//! src/telemetry.rs
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry, fmt::MakeWriter};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;

/// Compose multiple layers into a `tracing` subscriber.
pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync {
    // Fall back to having all tracing spans at INFO level or abive
    // if the RUST_LOG environment has not been set.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    // set-up tracing subscriber using settings above and
    // instantiate with `set_global_default`
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as a global default to process span data.
/// It should only be called once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our tracing subscriber
    LogTracer::init().expect("Failed to set tracing logger");
    set_global_default(subscriber).expect("Failed to set tracing subscriber.");
}
