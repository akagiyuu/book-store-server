mod delete;
mod get;
mod get_all;
mod insert;

use std::sync::Arc;

use axum::{Router, routing};

use crate::{middleware, state::ApiState};

pub use delete::*;
pub use get::*;
pub use get_all::*;
pub use insert::*;

pub fn build(state: Arc<ApiState>) -> Router<Arc<ApiState>> {
    Router::new()
        .route("/author", routing::post(insert_author))
        .route("/author/{id}", routing::delete(delete_author))
        .layer(axum::middleware::from_fn_with_state(
            state,
            middleware::admin_required,
        ))
        .route("/author", routing::get(get_all_author))
        .route("/author/{id}", routing::get(get_author))
}
