use std::sync::Arc;

use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::config::CONFIG;
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
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<CookieJar> {
    let user = database::user::get(&request.email, &state.database).await?;

    if !bcrypt::verify(request.password, &user.password).map_err(anyhow::Error::from)? {
        return Err(AuthError::InvalidLoginData.into());
    }

    let auth_ctx = AuthContext::new(user.id);
    let token = auth_ctx.encode()?;

    Ok(jar.add(Cookie::new(&CONFIG.token_cookie, token)))
}
