use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::Error as SqlxError;
use reqwest::Error as ReqwestError;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    SomeErrorKind,
    SqlxError(String),
    ReqwestError(String),
    SupabaseError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Self {
        Error::SqlxError(err.to_string())
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::ReqwestError(err.to_string())
    }
}