use std::string::FromUtf8Error;

use axum::Json;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use serde_json::json;


pub type ApiResult<T = ()> = std::result::Result<ApiResponse<T>, Error>;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: usize,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: usize, message: String, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
    pub fn ok(data: Option<T>) -> Self {
        Self {
            code: 20000,
            message: "ok".to_string(),
            data,
        }
    }

    pub fn fail(code: usize, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn ok_with_msg(data: Option<T>, message: String) -> Self {
        Self {
            code: 20000,
            message,
            data,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
    where T: Serialize
{
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::error::Error),

    #[error("{0}")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("{0}")]
    DbError(#[from] DbErr),

    #[error("{0}")]
    AuthError(#[from] AuthError),

    #[error("{0}")]
    InternalServerError(String),
}


#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("{0}")]
    Error(String),
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token type")]
    InvalidTokenType,
}


impl Error {
    fn get_code(&self) -> (StatusCode, u16) {
        match self {
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40001),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40002),
            Error::AuthError(_) => (StatusCode::UNAUTHORIZED, 40003),
            Error::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50000),
            Error::SerdeJsonError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50003),
            Error::FromUtf8Error(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50004),
            Error::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50005),
        }
    }
    pub fn bad_request() -> Self {
        Error::BadRequest(BadRequest {})
    }
    pub fn not_found() -> Self {
        Error::NotFound(NotFound {})
    }
    pub fn auth_error() -> Self { Error::AuthError(AuthError::Error("Auth Error".to_string())) }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::Error(e) => (StatusCode::UNAUTHORIZED, e),
            AuthError::TokenCreation => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidTokenType => (StatusCode::UNAUTHORIZED, self.to_string())
        }
            .into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_code();
        let msg = self.to_string();
        let body = Json(json!({"code": code, "message": msg}));
        (status_code, body).into_response()
    }
}


#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}
