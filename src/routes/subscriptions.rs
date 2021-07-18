//! src/routes/subscriptions.rs
use chrono::Utc;
use uuid::Uuid;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use log;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection` wrapped by web::Data
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved to database.");
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
