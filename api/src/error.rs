use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum Error {
    DbError(String),
    ValidateError(String),
    AuthenticateError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DbError(e) => write!(f, "Error DB error : {}", e),
            Error::ValidateError(e) => write!(f, "Validate error : {}", e),
            Error::AuthenticateError(e) => write!(f, "Authenticate error : {}", e)
        }
    }
}

// to adapt Box<dyn std::error::Error>
impl std::error::Error for Error{}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DbError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            Error::ValidateError(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            Error::AuthenticateError(e) => (StatusCode::FORBIDDEN, e).into_response()
        }
    }
}