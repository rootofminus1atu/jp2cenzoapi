use serde::{Deserialize, Serialize};
use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};


pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/random", get(get_random))
        .route("/:id", get(get_one))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    quote: String,
    translation: String
}

async fn get_all() -> impl IntoResponse {
    "all quotes"
}

async fn get_random() -> impl IntoResponse {
    "random quote"
}

async fn get_one() -> impl IntoResponse {
    "one quote"
}