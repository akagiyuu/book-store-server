use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    Result,
    database::{self, author::Author},
    state::ApiState,
};

#[utoipa::path(
    get,
    tag = "Author",
    path = "/author",
    responses(
        (status = 200, body = Vec<Author>)
    )
)]
pub async fn get_all_author(State(state): State<Arc<ApiState>>) -> Result<Json<Vec<Author>>> {
    let authors = database::author::get_all(&state.database).await?;

    Ok(Json(authors))
}
