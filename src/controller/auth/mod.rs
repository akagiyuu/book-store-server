mod google;
mod login;
mod register;

use std::sync::Arc;

use axum::{Router, routing};

use crate::state::ApiState;

pub use google::*;
pub use login::*;
pub use register::*;

pub fn build() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/auth/register", routing::post(register))
        .route("/auth/login", routing::post(login))
        .route("/auth/google", routing::get(google))
        .route("/auth/google/authorized", routing::get(authorized))
}
