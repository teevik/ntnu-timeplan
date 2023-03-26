use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use color_eyre::Report;
use tracing::error;

pub struct AppError(color_eyre::Report);

impl<Error> From<Error> for AppError
where
    Error: Into<Report>,
{
    fn from(error: Error) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{:?}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    }
}
