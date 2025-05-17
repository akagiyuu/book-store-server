use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, category::UpdateCategory},
    state::ApiState,
};

#[utoipa::path(
    patch,
    tag = "Category",
    path = "/category/{id}",
    params(
        ("id" = Uuid, Path, description = "Category id")
    ),
    request_body = UpdateCategory,
    security(("jwt_token" = []))
)]
pub async fn update_category(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCategory>,
) -> Result<()> {
    database::category::update(id, &req, &state.database).await?;

    Ok(())
}
