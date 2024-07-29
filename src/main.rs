use axum::{
    routing::get,
    Router,
};
use tracing_subscriber;

pub mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new()
        .route("/", get(|| async { 
            tracing::info!("hello sent!");
            "Hello, World!" 
        }))
        .nest("/api", api::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening");
    axum::serve(listener, app).await.unwrap();
}
