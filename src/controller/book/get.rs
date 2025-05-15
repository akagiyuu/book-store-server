use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, book::Book},
    state::ApiState,
};

#[utoipa::path(
    get,
    tag = "Book",
    path = "/book/:id",
    params(
        ("id" = Uuid, Path, description = "Book id")
    ),
    responses(
        (status = 200, body = Book)
    )
)]
pub async fn get(State(state): State<Arc<ApiState>>, Path(id): Path<Uuid>) -> Result<Json<Book>> {
    let book = database::book::get(id, &state.database).await?;

    Ok(Json(book))
}
