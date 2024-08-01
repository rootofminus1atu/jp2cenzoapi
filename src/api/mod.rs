use axum::Router;
use utoipa::OpenApi;
pub mod quotes;
pub mod images;
use crate::api::quotes::Quote;
use crate::helpers::utoipa_ext::nest_openapis_at_prefix;

pub fn routes() -> Router {
    Router::new()
        .nest("/quotes", quotes::routes())
        .nest("/images", images::routes())
}

#[derive(OpenApi)]
#[openapi(
    components(schemas(Quote))
)]
pub struct ApiDoc;


pub fn build_openapi() -> utoipa::openapi::OpenApi {
    let quotes_api = quotes::QuotesApi::openapi();
    let images_api = images::ImagesApi::openapi();

    nest_openapis_at_prefix(quotes_api, images_api, "/api")
}
