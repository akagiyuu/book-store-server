use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::database;
use crate::error::AuthError;
use crate::middleware::AuthContext;
use crate::{Result, state::ApiState};

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[utoipa::path (
    post,
    tag = "Auth",
    path = "/auth/login",
    request_body = LoginRequest,
)]
pub async fn login(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<LoginRequest>,
) -> Result<String> {
    let user = database::user::get(&request.email, &state.database).await?;

    if !bcrypt::verify(request.password, &user.password).map_err(anyhow::Error::from)? {
        return Err(AuthError::InvalidLoginData.into());
    }

    let auth_ctx = AuthContext::new(user.id);
    auth_ctx.encode()
}
