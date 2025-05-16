mod delete;
mod get;
mod get_all;
mod insert;
mod update;

use std::sync::Arc;

use axum::{Router, routing};

use crate::{middleware, state::ApiState};

pub use delete::*;
pub use get::*;
pub use get_all::*;
pub use insert::*;
pub use update::*;

pub fn build(state: Arc<ApiState>) -> Router<Arc<ApiState>> {
    Router::new()
        .route("/author", routing::post(insert))
        .route("/author/{id}", routing::patch(update))
        .route("/author/{id}", routing::delete(delete))
        .layer(axum::middleware::from_fn_with_state(
            state,
            middleware::admin_required,
        ))
        .route("/author", routing::get(get_all))
        .route("/author/{id}", routing::get(get))
}
