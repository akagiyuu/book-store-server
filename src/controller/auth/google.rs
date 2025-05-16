use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse, reqwest::async_http_client};
use serde::Deserialize;

use crate::{
    Result, config::CONFIG, database, error::AuthError, middleware::AuthContext, state::ApiState,
};

pub async fn google(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let (auth_url, _) = state
        .google_oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct User {
    email: String,
    family_name: Option<String>,
    given_name: Option<String>,
}

pub async fn authorized(
    Query(query): Query<AuthRequest>,
    jar: CookieJar,
    State(state): State<Arc<ApiState>>,
) -> Result<CookieJar> {
    let token = state
        .google_oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .map_err(|error| {
            tracing::error!(error =? error);

            AuthError::InvalidAuthToken
        })?;

    let user = state
        .http_client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|error| {
            tracing::error!(error =? error);

            AuthError::InvalidAuthToken
        })?
        .json::<User>()
        .await
        .map_err(|error| {
            tracing::error!(error =? error);

            AuthError::InvalidLoginData
        })?;

    let id = if database::user::is_existed(&user.email, &state.database).await? {
        database::user::get(&user.email, &state.database).await?.id
    } else {
        database::user::insert(
            &user.email,
            None,
            &user.family_name.unwrap_or_default(),
            &user.given_name.unwrap_or_default(),
            &state.database,
        )
        .await?
    };

    let auth_ctx = AuthContext::new(id);
    let token = auth_ctx.encode()?;

    Ok(jar.add(Cookie::new(&CONFIG.token_cookie, token)))
}
