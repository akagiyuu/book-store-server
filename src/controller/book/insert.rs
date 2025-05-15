use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;
use uuid::Uuid;

use crate::{Result, database, state::ApiState};

#[derive(Deserialize)]
pub struct BookInsertRequest {
    isbn: String,
    title: String,
    description: String,
    authors: Vec<Uuid>,
    categories: Vec<Uuid>,
}

#[utoipa::path(post, tag = "Book", path = "/book/:id")]
pub async fn insert(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<BookInsertRequest>,
) -> Result<()> {
    database::book::insert(
        &req.isbn,
        &req.title,
        &req.description,
        req.authors,
        req.categories,
        &state.database,
    )
    .await?;

    Ok(())
}
