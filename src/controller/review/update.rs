use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, review::UpdateReview},
    state::ApiState,
};

#[utoipa::path(
    patch,
    tag = "Book",
    path = "/book/:book_id/review/:user_id",
    params(
        ("book_id" = Uuid, Path, description = "Book id"),
        ("user_id" = Uuid, Path, description = "User id")
    ),
    request_body = UpdateReview,
    security(("jwt_token" = []))
)]
pub async fn update(
    State(state): State<Arc<ApiState>>,
    Path(book_id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateReview>,
) -> Result<()> {
    database::review::update(book_id, user_id, &req, &state.database)
        .await
        .unwrap();

    Ok(())
}
