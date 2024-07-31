use serde::{de::value, Deserialize, Serialize};
use axum::{extract::{rejection::PathRejection, Path}, response::IntoResponse, routing::get, Extension, Json, Router};
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
    pub id: i64,
    pub quote: String,
    pub translation: String
}

async fn get_all(Extension(db): Extension<PgPool>) -> Result<Json<Vec<Quote>>, Error> {
    let quotes = query_as::<_, Quote>("SELECT * FROM quote")
        .fetch_all(&db)
        .await?;

    Ok(Json(quotes))
}


async fn get_random(Extension(db): Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    let quote = query_as::<_, Quote>("SELECT * FROM quote ORDER BY RANDOM() LIMIT 1")
        .fetch_one(&db)
        .await?;

    Ok(Json(quote))
}

async fn get_one(Extension(db): Extension<PgPool>, Path(quote_id): Path<String>) -> Result<impl IntoResponse, Error> {
    let quote_id = quote_id.parse::<i64>()
        .map_err(|_| Error::InvalidQuoteId { id: quote_id })?;

    let quote = query_as::<_, Quote>("SELECT * FROM quote WHERE id = $1")
        .bind(&quote_id)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::QuoteWithIdNotFound { id: quote_id })?;

    Ok(Json(quote))
}