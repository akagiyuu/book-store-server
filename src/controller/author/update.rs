use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, author::UpdateAuthor},
    state::ApiState,
};

#[utoipa::path(
    patch,
    tag = "Author",
    path = "/author",
    params(
        ("id" = Uuid, Path, description = "Author id")
    ),
    request_body = UpdateAuthor,
    security(("jwt_token" = []))
)]
pub async fn update(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAuthor>,
) -> Result<()> {
    database::author::update(id, &req, &state.database).await?;

    Ok(())
}
