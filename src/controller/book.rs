use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing,
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, book::Book},
    state::ApiState,
};

#[utoipa::path(get, tag = "Book", path = "/book/:id")]
pub async fn get(State(state): State<Arc<ApiState>>, Path(id): Path<Uuid>) -> Result<Json<Book>> {
    let book = database::book::get(id, &state.database).await?;

    Ok(Json(book))
}

pub fn build() -> Router<Arc<ApiState>> {
    Router::new().route("/book/{id}", routing::get(get))
}
