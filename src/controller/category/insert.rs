use std::sync::Arc;

use axum::{Json, extract::State};

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
    security(("jwt_token" = []))
)]
pub async fn insert(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<InsertCategory>,
) -> Result<()> {
    database::category::insert(&req, &state.database).await?;

    Ok(())
}
