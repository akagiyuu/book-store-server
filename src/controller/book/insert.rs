use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{Result, database::book, state::ApiState};

#[utoipa::path(
    post,
    tag = "Book",
    path = "/book",
    request_body = book::InsertBook,
    security(("jwt_token" = []))
)]
pub async fn insert(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<book::InsertBook>,
) -> Result<()> {
    book::insert(req, &state.database).await?;

    Ok(())
}
