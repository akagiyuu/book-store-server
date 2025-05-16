use std::sync::Arc;

use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{Result, database, state::ApiState};

#[utoipa::path(
    delete,
    tag = "Book",
    path = "/book/:book_id/review/:user_id",
    params(
        ("book_id" = Uuid, Path, description = "Book id"),
        ("user_id" = Uuid, Path, description = "User id"),
    ),
    security(("jwt_token" = []))
)]
pub async fn delete(
    State(state): State<Arc<ApiState>>,
    Path(book_id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
) -> Result<()> {
    database::review::delete(book_id, user_id, &state.database)
        .await
        .unwrap();

    Ok(())
}
