use std::sync::Arc;

use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::Validation;
use uuid::Uuid;

use crate::{ApiState, Error, config::JWT_DECODING_KEY};

pub struct JWTClaims {
    pub sub: Uuid,
    pub exp: u64,
}

impl FromRequestParts<Arc<ApiState>> for JWTClaims {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ApiState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let token = bearer.token();
        let token = jsonwebtoken::decode(token, &JWT_DECODING_KEY, &Validation::default())?;

        Ok(token.claims)
    }
}
