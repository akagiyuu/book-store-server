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
    path = "/book/:book_id/review",
    params(
        ("book_id" = Uuid, Path, description = "Book id"),
    ),
    request_body = UpdateReview,
    security(("jwt_token" = []))
)]
pub async fn update(
    State(state): State<Arc<ApiState>>,
    auth_ctx: AuthContext,
    Path(book_id): Path<Uuid>,
    Json(req): Json<UpdateReview>,
) -> Result<()> {
    database::review::update(book_id, auth_ctx.sub, &req, &state.database).await?;

    Ok(())
}
