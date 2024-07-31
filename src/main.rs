use axum::{routing::get, Extension, Router};
use tracing_subscriber;
use dotenvy::dotenv;
use sqlx::PgPool;

pub mod api;
pub mod error;
pub mod storage;

use storage::Storage;


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
        .route("/", get(|| async { 
            tracing::info!("hello sent!");
            "Hello, World!" 
        }))
        .nest("/api", api::routes())
        .layer(Extension(db))
        .layer(Extension(storage));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening");
    axum::serve(listener, app).await.unwrap();
}
