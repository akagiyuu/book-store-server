use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, review::InsertReview},
    middleware::AuthContext,
    state::ApiState,
};

#[utoipa::path(
    post,
    tag = "Book",
    path = "/book/:book_id/review",
    params(
        ("book_id" = Uuid, Path, description = "Book id"),
    ),
    request_body = InsertReview,
    security(("jwt_token" = []))
)]
pub async fn insert_review(
    State(state): State<Arc<ApiState>>,
    auth_ctx: AuthContext,
    Path(book_id): Path<Uuid>,
    Json(req): Json<InsertReview>,
) -> Result<()> {
    database::review::insert(book_id, auth_ctx.sub, &req, &state.database).await?;

    Ok(())
}
