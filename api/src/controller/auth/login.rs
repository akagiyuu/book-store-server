use std::sync::Arc;

use axum::{Json, extract::State};
use jsonwebtoken::Header;
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::config::KEYS;
use crate::middleware::AuthContext;
use crate::{Result, config::CONFIG, state::ApiState};

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

struct User {
    id: Uuid,
    password: String,
}

async fn get_user(email: &str, database: &PgPool) -> Result<User> {
    let password = sqlx::query_as!(
        User,
        "SELECT id, password FROM users WHERE email = $1 LIMIT 1",
        email
    )
    .fetch_one(database)
    .await
    .unwrap();

    Ok(password)
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
    let user = get_user(&request.email, &state.database).await?;

    if !bcrypt::verify(request.password, &user.password).unwrap() {
        panic!("Invalid password");
    }

    let auth_ctx = AuthContext {
        sub: user.id,
        exp: CONFIG.jwt_expired_in,
    };
    let token = jsonwebtoken::encode(&Header::default(), &auth_ctx, &KEYS.encoding).unwrap();

    Ok(token)
}
