use std::sync::Arc;

use axum::{Json, extract::State};
use uuid::Uuid;

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
    responses(
        (status = 200, body = Uuid)
    ),
    security(("jwt_token" = []))
)]
pub async fn insert_author(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<InsertAuthor>,
) -> Result<Json<Uuid>> {
    let id = database::author::insert(&req, &state.database).await?;

    Ok(Json(id))
}
