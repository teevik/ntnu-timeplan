use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rspc::ErrorCode;
use std::fmt::{Display, Formatter};
use thiserror::Error;

// TODO: Just use anyhow again when rspc allows for generic errors

#[derive(Error, Debug)]
pub enum AppError {
    ReqwestError(#[from] reqwest::Error),

    ParsingError,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal server error")
    }
}

impl From<AppError> for rspc::Error {
    fn from(error: AppError) -> Self {
        let message = "Internal server error".to_owned();

        match error {
            AppError::ReqwestError(cause) => {
                rspc::Error::with_cause(ErrorCode::InternalServerError, message, cause)
            }
            AppError::ParsingError => rspc::Error::new(ErrorCode::InternalServerError, message),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
