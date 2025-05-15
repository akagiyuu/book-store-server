use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{Result, database::book, state::ApiState};

#[utoipa::path(patch, tag = "Book", path = "/book")]
pub async fn update(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<book::Update>,
) -> Result<()> {
    book::update(id, &req, &state.database).await?;

    Ok(())
}
