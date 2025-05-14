use std::sync::Arc;

use axum::{
    RequestPartsExt,
    body::Body,
    extract::FromRequestParts,
    http::{Request, Response, request::Parts},
    middleware::Next,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::Validation;
use serde::Deserialize;
use uuid::Uuid;

use crate::{Result, config::KEYS, error::Error, state::ApiState};

#[derive(Debug, sqlx::Type, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[sqlx(type_name = "role", rename_all = "snake_case")]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Deserialize)]
pub struct AuthContext {
    pub sub: Uuid,
    pub role: Role,
}

impl FromRequestParts<Arc<ApiState>> for AuthContext {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, _: &Arc<ApiState>) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .unwrap();

        let token = bearer.token();
        let token =
            jsonwebtoken::decode::<AuthContext>(token, &KEYS.decoding, &Validation::default())
                .unwrap();

        Ok(token.claims)
    }
}

pub async fn auth_required(
    role: Role,
) -> impl AsyncFn(AuthContext, Request<Body>, Next) -> Result<Response<Body>> {
    async move |auth_ctx: AuthContext, req: Request<Body>, next: Next| -> Result<Response<Body>> {
        if auth_ctx.role >= role {
            return Err(Error::Auth {
                message: "Unauthorized".to_string(),
            });
        }

        Ok(next.run(req).await)
    }
}
