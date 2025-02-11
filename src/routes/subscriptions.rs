use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, subscribed_at)
        VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        form.email,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
