use serde::{Deserialize, Serialize};
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::{query_as, PgPool};
use tracing::info;

use crate::error::Error;


pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/random", get(get_random))
        .route("/:id", get(get_one))
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Quote {
    quote: String,
    translation: String
}

async fn get_all(Extension(db): Extension<PgPool>) -> Result<Json<Vec<Quote>>, Error> {
    let h = query_as::<_, Quote>("SELECT * FROM quote")
        .fetch_all(&db)
        .await?;

    Ok(Json(h))
}


async fn get_random(Extension(db): Extension<PgPool>) -> impl IntoResponse {
    "random quote"
}

async fn get_one(Extension(db): Extension<PgPool>) -> impl IntoResponse {
    "one quote"
}