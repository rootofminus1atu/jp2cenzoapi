use askama::Template;
use axum::response::{Html, IntoResponse};
use reqwest::StatusCode;

use crate::api::quotes::Quote;



#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;


pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T> 
where
    T: Template
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(why) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", why)
            ).into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "quotes.html")]
pub struct QuotesTemplate {
    pub quotes: Vec<Quote>
}
