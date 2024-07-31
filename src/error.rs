use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use sqlx::Error as SqlxError;
use reqwest::Error as ReqwestError;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    QuoteWithIdNotFound { id: i64 },
    InvalidQuoteId { id: String },
    SqlxError(String),
    ReqwestError(String),
    InternalServerError
}

pub struct ErrorData {
    status_code: StatusCode,
    client_error_type: ClientErrorType,
    error_message: String
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientErrorType {
    QUOTE_NOT_FOUND,
    INVALID_QUOTE_ID,
    SERVICE_ERROR,
}


impl Error {
    fn error_data(&self) -> ErrorData {
        match self {
            Self::QuoteWithIdNotFound { id } => ErrorData {
                status_code: StatusCode::NOT_FOUND,
                client_error_type: ClientErrorType::QUOTE_NOT_FOUND,
                error_message: format!("Quote with id {} not found", id),
            },
            Self::InvalidQuoteId { id } => ErrorData {
                status_code: StatusCode::BAD_REQUEST,
                client_error_type: ClientErrorType::INVALID_QUOTE_ID,
                error_message: format!("{} is not a valid quote id", id),
            },
            Self::SqlxError(_) | Self::ReqwestError(_) | Self::InternalServerError => ErrorData {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                client_error_type: ClientErrorType::SERVICE_ERROR,
                error_message: "Internal server error".to_string(),
            },
        }
    }

    // pub fn 
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let error_data = self.error_data();
        let body = serde_json::json!({
            "error": {
                "type": error_data.client_error_type.as_ref(),
                "message": error_data.error_message,
            }
        });
        (error_data.status_code, Json(body)).into_response()
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




// boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
