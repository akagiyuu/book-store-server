use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, review::Review},
    state::ApiState,
};

#[utoipa::path(
    get,
    tag = "Book",
    path = "/book/:id/review",
    params(
        ("id" = Uuid, Path, description = "Book id"),
    ),
    responses(
        (status = 200, body = Vec<Review>)
    )
)]
pub async fn get_review_by_book(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<Review>>> {
    let reviews = database::review::get_by_book(id, &state.database).await?;

    Ok(Json(reviews))
}
