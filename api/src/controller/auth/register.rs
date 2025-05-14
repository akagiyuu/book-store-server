use std::sync::Arc;

use axum::{Json, extract::State};
use jsonwebtoken::Header;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::config::KEYS;
use crate::middleware::{AuthContext, Role};
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
    Json(request): Json<RegisterRequest>,
) -> Result<String> {
    let password = bcrypt::hash_with_salt(
        request.password.as_bytes(),
        CONFIG.bcrypt_cost,
        CONFIG.bcrypt_salt,
    )
    .unwrap()
    .to_string();

    let user = sqlx::query!(
        r#"
            INSERT INTO users(email, password, first_name, last_name)
            VALUES($1, $2, $3, $4)
            RETURNING id, role as "role!: Role"
        "#,
        request.email,
        password,
        request.first_name,
        request.last_name,
    )
    .fetch_one(&state.database)
    .await
    .unwrap();

    let auth_ctx = AuthContext {
        sub: user.id,
        exp: CONFIG.jwt_expired_in,
    };
    let token = jsonwebtoken::encode(&Header::default(), &auth_ctx, &KEYS.encoding).unwrap();

    Ok(token)
}
