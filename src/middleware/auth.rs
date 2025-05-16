use std::sync::Arc;

use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    Result,
    config::{CONFIG, KEYS},
    error::AuthError,
    state::ApiState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthContext {
    pub sub: Uuid,
    pub exp: u64,
}

impl AuthContext {
    pub fn new(id: Uuid) -> Self {
        Self {
            sub: id,
            exp: CONFIG.jwt_expired_in,
        }
    }

    pub fn encode(&self) -> Result<String> {
        let token = jsonwebtoken::encode(&Header::default(), self, &KEYS.encoding).unwrap();

        Ok(token)
    }
}

impl FromRequestParts<Arc<ApiState>> for AuthContext {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, _: &Arc<ApiState>) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let token = bearer.token();
        let token =
            jsonwebtoken::decode::<AuthContext>(token, &KEYS.decoding, &Validation::default())
                .map_err(|error| {
                    tracing::error!(error = ?error);
                    AuthError::InvalidAuthToken
                })?;

        Ok(token.claims)
    }
}

#[derive(Debug, sqlx::Type, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[sqlx(type_name = "role", rename_all = "snake_case")]
pub enum Role {
    User,
    Admin,
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

macro_rules! auth_required {
    ($name:ident, $role: expr) => {
        pub async fn $name(
            auth_ctx: AuthContext,
            State(state): State<Arc<ApiState>>,
            req: Request,
            next: Next,
        ) -> Result<Response> {
            if get_role(auth_ctx.sub, &state.database).await? >= $role {
                return Err(AuthError::MissingPermission.into());
            }

            Ok(next.run(req).await)
        }
    };
}

auth_required!(admin_required, Role::Admin);
auth_required!(user_required, Role::User);
