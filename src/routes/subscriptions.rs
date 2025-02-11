use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgConnection;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
}

pub async fn subscribe(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
