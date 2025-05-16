#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authentication token")]
    MissingAuthToken,

    #[error("Invalid authentication token")]
    InvalidAuthToken,

    #[error("Missing required permission")]
    MissingPermission,
}
