use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    Result,
    database::{self, author::InsertAuthor},
    state::ApiState,
};

#[utoipa::path(
    post,
    tag = "Author",
    path = "/author",
    request_body = InsertAuthor,
    security(("jwt_token" = []))
)]
pub async fn insert(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<InsertAuthor>,
) -> Result<()> {
    database::author::insert(&req, &state.database).await?;

    Ok(())
}
