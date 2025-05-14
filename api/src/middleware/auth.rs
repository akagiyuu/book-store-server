use std::sync::Arc;

use axum::{
    RequestPartsExt,
    body::Body,
    extract::{FromRequestParts, State},
    http::{Request, Response, request::Parts},
    middleware::Next,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{Result, config::KEYS, error::Error, state::ApiState};

#[derive(Debug, sqlx::Type, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[sqlx(type_name = "role", rename_all = "snake_case")]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthContext {
    pub sub: Uuid,
    pub exp: u64,
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

async fn get_role(id: Uuid, database: &PgPool) -> Result<Role> {
    let role = sqlx::query_scalar!(
        r#"SELECT role as "role!: Role" FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_one(database)
    .await
    .unwrap();

    Ok(role)
}

pub async fn auth_required(
    role: Role,
) -> impl AsyncFn(AuthContext, State<Arc<ApiState>>, Request<Body>, Next) -> Result<Response<Body>>
{
    async move |auth_ctx: AuthContext,
                State(state): State<Arc<ApiState>>,
                req: Request<Body>,
                next: Next|
                -> Result<Response<Body>> {
        if get_role(auth_ctx.sub, &state.database).await? >= role {
            return Err(Error::Auth {
                message: "Unauthorized".to_string(),
            });
        }

        Ok(next.run(req).await)
    }
}
