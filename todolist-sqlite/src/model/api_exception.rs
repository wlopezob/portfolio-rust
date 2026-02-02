use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;
use std::sync::PoisonError;

#[derive(Error, Debug)]
pub enum ApiException {
    #[error("{0}")]
    InternalError(String),
    
    #[error("Database lock error: {0}")]
    DatabaseLockError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),
}

impl ApiException {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiException::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::DatabaseLockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::NotFound(_) => StatusCode::NOT_FOUND,
            ApiException::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn internal_error(msg: impl Into<String>) -> Self {
        ApiException::InternalError(msg.into())
    }
}

// convert automatic from PoisonError to ApiException
impl<T> From<PoisonError<T>> for ApiException {
    fn from(err: PoisonError<T>) -> Self {
        ApiException::DatabaseLockError(err.to_string())
    }
}

#[derive(Serialize, ToSchema)]
pub struct ApiExceptionResponseMessage {
    pub code: String,
    pub message: String,
}

impl IntoResponse for ApiException {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        (
            status_code,
            Json(ApiExceptionResponseMessage {
                code: status_code.as_str().to_string(),
                message: self.to_string(),
            }),
        ).into_response()
    }
    
}