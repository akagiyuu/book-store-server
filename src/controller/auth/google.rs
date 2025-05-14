use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;

use crate::{Result, database, middleware::AuthContext, state::ApiState};

pub async fn google(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let (auth_url, _) = state
        .google_oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
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
    family_name: String,
    given_name: String,
}

pub async fn authorized(
    Query(query): Query<AuthRequest>,
    State(state): State<Arc<ApiState>>,
) -> Result<impl IntoResponse> {
    let token = state
        .google_oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(&state.http_client)
        .await
        .unwrap();
    let user = state
        .http_client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    let id = if !database::user::is_existed(&user.email, &state.database).await? {
        database::user::get(&user.email, &state.database).await?.id
    } else {
        database::user::insert(
            &user.email,
            None,
            &user.family_name,
            &user.given_name,
            &state.database,
        )
        .await?
    };

    let auth_ctx = AuthContext::new(id);
    auth_ctx.encode()
}
