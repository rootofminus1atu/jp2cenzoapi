use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::{query_as, query_scalar, PgPool};
use utoipa::OpenApi;
use crate::{error::Error, model::CountResponse, storage::Object, Storage};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/random", get(get_random))
        .route("/count", get(get_count))
}

#[derive(OpenApi)]
#[openapi(
    paths(get_all, get_random),
    // components(schemas(??? should be the img object but links will do for now)),
    tags(
        (name = "images", description = "Images API endpoints")
    )
)]
pub struct ImagesApi;

#[utoipa::path(
    get,
    path = "/",
    tag = "images",
    responses(
        (status = 200, description = "Get all images", body = [String])
    )
)]
async fn get_all(Extension(db): Extension<PgPool>, Extension(storage): Extension<Storage>) -> Result<impl IntoResponse, Error> {
    let img_links = query_as::<_, Object>("SELECT * FROM storage.objects WHERE bucket_id = $1 AND metadata->>'mimetype' != 'application/octet-stream'")
        .bind(&storage.bucket_id)
        .fetch_all(&db)
        .await?
        .iter()
        .map(|img| img.to_link(&storage))
        .collect::<Vec<_>>();

    Ok(Json(img_links))
}

#[utoipa::path(
    get,
    path = "/random",
    tag = "images",
    responses(
        (status = 200, description = "Get a random image", body = String)
    )
)]
async fn get_random(Extension(db): Extension<PgPool>, Extension(storage): Extension<Storage>) -> Result<impl IntoResponse, Error> {
    let img_link = query_as::<_, Object>("SELECT * FROM storage.objects WHERE bucket_id = $1 AND metadata->>'mimetype' != 'application/octet-stream' ORDER BY RANDOM() LIMIT 1")
        .bind(&storage.bucket_id)
        .fetch_one(&db)
        .await?
        .to_link(&storage);

    Ok(Json(img_link))
}


async fn get_count(Extension(db): Extension<PgPool>, Extension(storage): Extension<Storage>) -> Result<impl IntoResponse, Error> {
    let count = query_scalar("SELECT COUNT(*) FROM storage.objects WHERE bucket_id = $1 AND metadata->>'mimetype' != 'application/octet-stream'")
        .bind(&storage.bucket_id)
        .fetch_one(&db)
        .await?;

    Ok(Json(CountResponse { count }))
}