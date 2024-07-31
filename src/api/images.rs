use serde::{Deserialize, Serialize};
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::{query_as, PgPool};
use tracing::info;
use crate::{error::Error, Storage};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/random", get(get_random))
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
struct Object {
    name: String,
    bucket_id: String
}

impl Object {
    pub fn to_link(&self, storage_data: &Storage) -> String {
        format!("{}/storage/v1/object/public/{}/{}", storage_data.supabase_url, storage_data.bucket_id, self.name)
    }
}

async fn get_all(Extension(db): Extension<PgPool>, Extension(storage): Extension<Storage>) -> Result<impl IntoResponse, Error> {
    let img_links = query_as::<_, Object>("SELECT * FROM storage.objects WHERE bucket_id = $1")
        .bind(&storage.bucket_id)
        .fetch_all(&db)
        .await?
        .iter()
        .map(|img| img.to_link(&storage))
        .collect::<Vec<_>>();

    Ok(Json(img_links))
}

async fn get_random(Extension(db): Extension<PgPool>, Extension(storage): Extension<Storage>) -> Result<impl IntoResponse, Error> {
    let img_link = query_as::<_, Object>("SELECT * FROM storage.objects WHERE bucket_id = $1 ORDER BY RANDOM() LIMIT 1")
        .bind(&storage.bucket_id)
        .fetch_one(&db)
        .await?
        .to_link(&storage);

    Ok(Json(img_link))
}
