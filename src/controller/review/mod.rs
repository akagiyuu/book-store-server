mod delete;
mod get;
mod get_by_book;
mod insert;
mod judge;
mod update;

use std::sync::Arc;

use axum::{Router, routing};

use crate::{middleware, state::ApiState};

pub use delete::*;
pub use get::*;
pub use get_by_book::*;
pub use insert::*;
pub use judge::*;
pub use update::*;

pub fn build(state: Arc<ApiState>) -> Router<Arc<ApiState>> {
    Router::new()
        .route("/book/{book_id}/review", routing::post(insert_review))
        .route("/book/{book_id}/review", routing::patch(update_review))
        .route("/book/{book_id}/review", routing::delete(delete_review))
        .layer(axum::middleware::from_fn_with_state(
            state,
            middleware::user_required,
        ))
        .route("/book/{book_id}/review", routing::get(get_review_by_book))
        .route("/book/{book_id}/review/{user_id}", routing::get(get_review))
        .route(
            "/book/{book_id}/review/{user_id}/judge",
            routing::get(judge),
        )
}
