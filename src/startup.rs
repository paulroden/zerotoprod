//! src/startup.rs

use std::net::TcpListener;
use actix_web::dev::{Server};
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;
use sqlx::PgPool;
use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Wrap the db connection in a smart pointer
    let connection = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // Middleware for logging added using `wrap` method
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    // no .await here!
    Ok(server)
}
