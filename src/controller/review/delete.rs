use std::sync::Arc;

use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{Result, database, middleware::AuthContext, state::ApiState};

#[utoipa::path(
    delete,
    tag = "Review",
    path = "/review/{id}",
    params(
        ("id" = Uuid, Path, description = "Review id"),
    ),
    security(("jwt_token" = []))
)]
pub async fn delete_review(
    State(state): State<Arc<ApiState>>,
    auth_ctx: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<()> {
    database::review::delete(id, auth_ctx.sub, &state.database).await?;

    Ok(())
}
