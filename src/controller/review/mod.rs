mod delete;
mod get;
mod get_by_book;
mod insert;
mod update;

use std::sync::Arc;

use axum::{Router, routing};

use crate::{middleware, state::ApiState};

pub use delete::*;
pub use get::*;
pub use get_by_book::*;
pub use insert::*;
pub use update::*;

pub fn build(state: Arc<ApiState>) -> Router<Arc<ApiState>> {
    Router::new()
        .route("/book/{book_id}/review/{user_id}", routing::post(insert))
        .route("/book/{book_id}/review/{user_id}", routing::patch(update))
        .route("/book/{book_id}/review/{user_id}", routing::delete(delete))
        .layer(axum::middleware::from_fn_with_state(
            state,
            middleware::user_required,
        ))
        .route("/book/{book_id}/review/{user_id}", routing::get(get))
        .route("/book/{book_id}/review", routing::get(get_by_book))
}
