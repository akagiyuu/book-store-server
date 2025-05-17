use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{Result, config::CONFIG, database, state::ApiState};

#[derive(Serialize, ToSchema)]
pub struct Judgement {
    pub score: i32,
    pub reason: String,
}

#[utoipa::path(
    get,
    tag = "Review",
    path = "/review/{id}/judge",
    params(
        ("id" = Uuid, Path, description = "Review id"),
    ),
    responses(
        (status = 200, body = Judgement)
    )
)]
pub async fn judge(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Judgement>> {
    todo!()
    // let review = database::review::get(book_id, user_id, &state.database).await?;
    //
    // let ollama = Ollama::new(CONFIG.ollama_host.clone(), CONFIG.ollama_port);
    //
    // let model = "llama2:latest".to_string();
    // let prompt = format!(
    //     "Judge this review, outputing the score reprensent the positive and negative of the reivew, also give me the reason. The review is: {}",
    //     review.content
    // );
    //
    // let response = ollama
    //     .generate(GenerationRequest::new(model, prompt))
    //     .await
    //     .unwrap()
    //     .response;
    //
    // let (score_raw, reason) = response.split_once("\n\n").unwrap();
    // let score = score_raw.parse().unwrap();
    //
    // Ok(Json(Judgement {
    //     score,
    //     reason: reason.to_string(),
    // }))
}
