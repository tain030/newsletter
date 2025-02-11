use serde::Deserialize;

pub mod configuration;
pub mod routes;
pub mod startup;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
}
