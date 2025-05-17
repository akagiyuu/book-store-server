mod delete;
mod get;
mod get_all;
mod insert;
mod update;

use std::sync::Arc;

use axum::{Router, routing};

use crate::{
    middleware,
    state::ApiState,
};

pub use delete::*;
pub use get::*;
pub use get_all::*;
pub use insert::*;
pub use update::*;

pub fn build(state: Arc<ApiState>) -> Router<Arc<ApiState>> {
    Router::new()
        .route("/book", routing::post(insert_book))
        .route("/book/{id}", routing::patch(update_book))
        .route("/book/{id}", routing::delete(delete_book))
        .layer(axum::middleware::from_fn_with_state(
            state,
            middleware::admin_required,
        ))
        .route("/book", routing::get(get_all_book))
        .route("/book/{id}", routing::get(get_book))
}
