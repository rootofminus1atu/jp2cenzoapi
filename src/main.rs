use axum::{response::IntoResponse, routing::get, Extension, Router};
use tracing::info;
use tracing_subscriber;
use dotenvy::dotenv;
use sqlx::PgPool;

pub mod api;
pub mod error;
pub mod storage;
pub mod template;
pub mod quote_form;

use storage::Storage;
use template::{HtmlTemplate, IndexTemplate}; 


#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let db_str = std::env::var("DATABASE_URL")
        .expect("no db url found :(");
    let bucket_id = std::env::var("BUCKET_ID")
        .expect("no bucket name/id found :(");
    let supabase_url = std::env::var("SUPABASE_URL")
        .expect("no supabase api url found :(");

    let db = PgPool::connect(&db_str).await
        .expect("couldnt connect to db");

    let storage = Storage::new(bucket_id, supabase_url);

    let app = Router::new()
        .route("/", get(index))
        .nest("/api", api::routes())
        .layer(Extension(db))
        .layer(Extension(storage));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}