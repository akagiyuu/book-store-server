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
        .route("/category", routing::post(insert_category))
        .route("/category/{id}", routing::patch(update_category))
        .route("/category/{id}", routing::delete(delete_category))
        .layer(axum::middleware::from_fn_with_state(
            state,
            middleware::admin_required,
        ))
        .route("/category", routing::get(get_all_category))
        .route("/category/{id}", routing::get(get_category))
}
