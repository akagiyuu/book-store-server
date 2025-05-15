use std::sync::Arc;

use axum::{Json, extract::State};
use futures::TryStreamExt;

use crate::{
    Result,
    database::{self, book::Book},
    state::ApiState,
};

#[utoipa::path(get, tag = "Book", path = "/book")]
pub async fn get_all(State(state): State<Arc<ApiState>>) -> Result<Json<Vec<Book>>> {
    let book = database::book::get_all(&state.database)
        .try_collect()
        .await?;

    Ok(Json(book))
}
