use std::sync::Arc;

use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{Result, database, state::ApiState};

#[utoipa::path(
    delete,
    tag = "Book",
    path = "/book/:id",
    params(
        ("id" = Uuid, Path, description = "Book id")
    ),
    security(("jwt_token" = []))
)]
pub async fn delete(State(state): State<Arc<ApiState>>, Path(id): Path<Uuid>) -> Result<()> {
    database::book::delete(id, &state.database).await?;

    Ok(())
}
