use std::sync::Arc;

use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::database;
use crate::middleware::AuthContext;
use crate::{Result, config::CONFIG, state::ApiState};

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[utoipa::path (
    post,
    tag = "Auth",
    path = "/auth/register",
    request_body = RegisterRequest,
)]
pub async fn register(
    State(state): State<Arc<ApiState>>,
    jar: CookieJar,
    Json(request): Json<RegisterRequest>,
) -> Result<CookieJar> {
    let password = bcrypt::hash_with_salt(
        request.password.as_bytes(),
        CONFIG.bcrypt_cost,
        CONFIG.bcrypt_salt,
    )
    .unwrap()
    .to_string();

    let id = database::user::insert(
        &request.email,
        Some(password),
        &request.first_name,
        &request.last_name,
        &state.database,
    )
    .await?;

    let auth_ctx = AuthContext::new(id);
    let token = auth_ctx.encode()?;

    Ok(jar.add(Cookie::new(&CONFIG.token_cookie, token)))
}
