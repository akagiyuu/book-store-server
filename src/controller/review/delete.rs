use std::sync::Arc;

use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{Result, database, middleware::AuthContext, state::ApiState};

#[utoipa::path(
    delete,
    tag = "Book",
    path = "/book/{book_id}/review",
    params(
        ("book_id" = Uuid, Path, description = "Book id"),
    ),
    security(("jwt_token" = []))
)]
pub async fn delete_review(
    State(state): State<Arc<ApiState>>,
    auth_ctx: AuthContext,
    Path(book_id): Path<Uuid>,
) -> Result<()> {
    database::review::delete(book_id, auth_ctx.sub, &state.database).await?;

    Ok(())
}
