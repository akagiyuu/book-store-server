use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, category::Category},
    state::ApiState,
};

#[utoipa::path(
    get,
    tag = "Category",
    path = "/category/{id}",
    params(
        ("id" = Uuid, Path, description = "Category id")
    ),
    responses(
        (status = 200, body = Category)
    )
)]
pub async fn get_category(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Category>> {
    let category = database::category::get(id, &state.database).await?;

    Ok(Json(category))
}
