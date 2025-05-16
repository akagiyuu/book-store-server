use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, review::InsertReview},
    state::ApiState,
};

#[utoipa::path(
    post,
    tag = "Author",
    path = "/author",
    params(
        ("book_id" = Uuid, Path, description = "Book id"),
        ("user_id" = Uuid, Path, description = "User id"),
    ),
    request_body = InsertReview,
    security(("jwt_token" = []))
)]
pub async fn insert(
    State(state): State<Arc<ApiState>>,
    Path(book_id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<InsertReview>,
) -> Result<()> {
    database::review::insert(book_id, user_id, &req, &state.database).await?;

    Ok(())
}
