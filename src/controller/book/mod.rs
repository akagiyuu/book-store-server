mod delete;
mod get;
mod get_all;
mod insert;
mod update;

use std::sync::Arc;

use axum::{Router, routing};

use crate::state::ApiState;

pub use delete::*;
pub use get::*;
pub use get_all::*;
pub use insert::*;
pub use update::*;

pub fn build() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/book", routing::post(insert))
        .route("/book", routing::get(get_all))
        .route("/book/{id}", routing::get(get))
        .route("/book/{id}", routing::patch(update))
        .route("/book/{id}", routing::delete(delete))
}
