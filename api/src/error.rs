use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown error: {message}")]
    Auth { message: String },
    #[error("Unknown error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, response) = match self {
            Error::Auth { message } => (StatusCode::UNAUTHORIZED, ErrorResponse { message }),
            Error::Other(error) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: error.to_string(),
                },
            ),
        };

        (status, Json(response)).into_response()
    }
}
