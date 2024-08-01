use serde::{Deserialize, Serialize};
use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::{query_as, query_scalar, PgPool};
use utoipa::{OpenApi, ToSchema};
use crate::error::{Error, ErrorResponse, ErrorResponseDetails};
use crate::model::CountResponse;


pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/random", get(get_random))
        .route("/count", get(get_count))
        .route("/:id", get(get_one))
}

#[derive(OpenApi)]
#[openapi(
    paths(get_all, get_random, get_one),
    components(schemas(Quote), schemas(ErrorResponse), schemas(ErrorResponseDetails)),
    tags(
        (name = "quotes", description = "Quotes API endpoints"),
        (name = "images", description = "Images API endpoints")
    )
)]
pub struct QuotesApi;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, ToSchema)]
pub struct Quote {
    pub id: i64,
    pub quote: String,
    pub translation: String
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Get all quotes", body = [Quote])
    ),
    tag = "quotes"
)]
async fn get_all(Extension(db): Extension<PgPool>) -> Result<Json<Vec<Quote>>, Error> {
    let quotes = query_as::<_, Quote>("SELECT * FROM quote")
        .fetch_all(&db)
        .await?;

    Ok(Json(quotes))
}

#[utoipa::path(
    get,
    path = "/random",
    responses(
        (status = 200, description = "Get a random quote", body = Quote)
    ),
    tag = "quotes"
)]
async fn get_random(Extension(db): Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    let quote = query_as::<_, Quote>("SELECT * FROM quote ORDER BY RANDOM() LIMIT 1")
        .fetch_one(&db)
        .await?;

    Ok(Json(quote))
}

#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = 200, description = "Get a quote by ID", body = Quote),
        (status = 404, description = "Quote not found", body = ErrorResponse),
        (status = 400, description = "Invalid quote ID", body = ErrorResponse)
    ),
    tag = "quotes",
    params(
        ("id" = i64, Path, description = "Quote ID")
    )
)]
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

async fn get_count(Extension(db): Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    let count = query_scalar("SELECT COUNT(*) FROM quote")
        .fetch_one(&db)
        .await?;

    Ok(Json(CountResponse { count }))
}

