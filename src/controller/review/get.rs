use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, author::Author, review::Review},
    state::ApiState,
};

#[utoipa::path(
    get,
    tag = "Book",
    path = "/book/:book_id/review/:user_id",
    params(
        ("book_id" = Uuid, Path, description = "Book id"),
        ("user_id" = Uuid, Path, description = "User id"),
    ),
    responses(
        (status = 200, body = Author)
    )
)]
pub async fn get(
    State(state): State<Arc<ApiState>>,
    Path(book_id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Review>> {
    let review = database::review::get(book_id, user_id, &state.database).await?;

    Ok(Json(review))
}
