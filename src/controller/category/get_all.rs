use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    Result,
    database::{self, category::Category},
    state::ApiState,
};

#[utoipa::path(
    get,
    tag = "Category",
    path = "/category",
    responses(
        (status = 200, body = Vec<Category>)
    )
)]
pub async fn get_all(State(state): State<Arc<ApiState>>) -> Result<Json<Vec<Category>>> {
    let book = database::category::get_all(&state.database).await?;

    Ok(Json(book))
}
