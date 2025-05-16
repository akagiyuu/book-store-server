use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::typed_header::TypedHeaderRejection;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authentication token")]
    MissingAuthToken,

    #[error("Invalid authentication token")]
    InvalidAuthToken,

    #[error("Missing required permission")]
    MissingPermission,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(sqlx::Error),

    #[error("{0}")]
    Auth(#[from] AuthError),

    #[error("Unknown error: {0}")]
    Other(#[from] anyhow::Error),
}

impl From<TypedHeaderRejection> for Error {
    fn from(_: TypedHeaderRejection) -> Self {
        AuthError::MissingAuthToken.into()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, response) = match self {
            Error::Database(_) => (StatusCode::BAD_REQUEST, ErrorResponse {
                message: "Failed to execute query".to_string(),
            }),
            Error::Auth(error) => (StatusCode::UNAUTHORIZED, ErrorResponse {
                message: error.to_string(),
            }),
            Error::Other(error) => (StatusCode::BAD_REQUEST, ErrorResponse {
                message: error.to_string(),
            }),
        };

        (status, Json(response)).into_response()
    }
}
