mod delete;
mod get;
mod judge;
mod update;

use std::sync::Arc;

use axum::{Router, routing};

use crate::{middleware, state::ApiState};

pub use delete::*;
pub use get::*;
pub use judge::*;
pub use update::*;

pub fn build(state: Arc<ApiState>) -> Router<Arc<ApiState>> {
    Router::new()
        .route("/review/{id}", routing::patch(update_review))
        .route("/review/{id}", routing::delete(delete_review))
        .layer(axum::middleware::from_fn_with_state(
            state,
            middleware::user_required,
        ))
        .route("/review/{id}", routing::get(get_review))
        .route("/review/{id}/judge", routing::get(judge))
}
