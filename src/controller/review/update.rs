use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, review::UpdateReview},
    middleware::AuthContext,
    state::ApiState,
};

#[utoipa::path(
    patch,
    tag = "Book",
    path = "/review/{id}",
    params(
        ("id" = Uuid, Path, description = "Review id"),
    ),
    request_body = UpdateReview,
    security(("jwt_token" = []))
)]
pub async fn update_review(
    State(state): State<Arc<ApiState>>,
    auth_ctx: AuthContext,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateReview>,
) -> Result<()> {
    database::review::update(id, auth_ctx.sub, &req, &state.database).await?;

    Ok(())
}
