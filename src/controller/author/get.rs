use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, author::Author},
    state::ApiState,
};

#[utoipa::path(
    get,
    tag = "Author",
    path = "/author/{id}",
    params(
        ("id" = Uuid, Path, description = "Author id")
    ),
    responses(
        (status = 200, body = Author)
    )
)]
pub async fn get(State(state): State<Arc<ApiState>>, Path(id): Path<Uuid>) -> Result<Json<Author>> {
    let author = database::author::get(id, &state.database).await?;

    Ok(Json(author))
}
