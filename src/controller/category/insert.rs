use std::sync::Arc;

use axum::{Json, extract::State};
use uuid::Uuid;

use crate::{
    Result,
    database::{self, category::InsertCategory},
    state::ApiState,
};

#[utoipa::path(
    post,
    tag = "Category",
    path = "/category",
    request_body = InsertCategory,
    responses(
        (status = 200, body = Uuid)
    ),
    security(("jwt_token" = []))
)]
pub async fn insert_category(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<InsertCategory>,
) -> Result<Json<Uuid>> {
    let id = database::category::insert(&req, &state.database).await?;

    Ok(Json(id))
}
