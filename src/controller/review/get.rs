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
    tag = "Review",
    path = "/review/{id}",
    params(
        ("id" = Uuid, Path, description = "Review id"),
    ),
    responses(
        (status = 200, body = Review)
    )
)]
pub async fn get_review(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Review>> {
    let review = database::review::get(id, &state.database).await?;

    Ok(Json(review))
}
