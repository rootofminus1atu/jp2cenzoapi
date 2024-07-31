use axum::Router;

pub mod quotes;
pub mod images;

pub fn routes() -> Router {
    Router::new()
        .nest("/quotes", quotes::routes())
        .nest("/images", images::routes())
}
