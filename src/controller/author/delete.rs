use std::sync::Arc;

use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{Result, database, state::ApiState};

#[utoipa::path(
    delete,
    tag = "Author",
    path = "/author/{id}",
    params(
        ("id" = Uuid, Path, description = "Author id")
    ),
    security(("jwt_token" = []))
)]
pub async fn delete_author(State(state): State<Arc<ApiState>>, Path(id): Path<Uuid>) -> Result<()> {
    database::author::delete(id, &state.database).await?;

    Ok(())
}
