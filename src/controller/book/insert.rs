use std::sync::Arc;

use axum::{Json, extract::State};
use uuid::Uuid;

use crate::{Result, database::book, state::ApiState};

#[utoipa::path(
    post,
    tag = "Book",
    path = "/book",
    request_body = book::InsertBook,
    responses(
        (status = 200, body = Uuid)
    ),
    security(("jwt_token" = []))
)]
pub async fn insert_book(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<book::InsertBook>,
) -> Result<Json<Uuid>> {
    let id = book::insert(req, &state.database).await?;

    Ok(Json(id))
}
