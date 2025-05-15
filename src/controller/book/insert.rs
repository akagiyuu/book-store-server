use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{Result, database::book, state::ApiState};

#[utoipa::path(
    post,
    tag = "Book",
    path = "/book",
    request_body = book::Insert,
)]
pub async fn insert(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<book::Insert>,
) -> Result<()> {
    book::insert(req, &state.database).await?;

    Ok(())
}
