mod get;

use std::sync::Arc;

use axum::{Router, routing};

pub use get::*;

use crate::state::ApiState;

pub fn build() -> Router<Arc<ApiState>> {
    Router::new().route("/book/{id}", routing::get(get))
}
