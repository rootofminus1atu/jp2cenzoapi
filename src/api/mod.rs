use axum::Router;

pub mod quotes;

pub fn routes() -> Router {
    Router::new()
        .nest("/quotes", quotes::routes())
}
