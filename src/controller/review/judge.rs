use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use ollama_rs::{
    Ollama,
    generation::{
        completion::request::GenerationRequest,
        parameters::{FormatType, JsonSchema, JsonStructure},
    },
    models::ModelOptions,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{Result, config::CONFIG, database, state::ApiState};

#[derive(Deserialize, Serialize, ToSchema, JsonSchema)]
pub struct Judgement {
    pub positivity_score: f32,
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
    let review = database::review::get(id, &state.database).await?;

    let ollama = Ollama::new(CONFIG.ollama_host.clone(), CONFIG.ollama_port);

    let model = CONFIG.ollama_model.clone();
    let prompt = review.content;

    let format = FormatType::StructuredJson(JsonStructure::new::<Judgement>());
    let response = ollama
        .generate(
            GenerationRequest::new(model, prompt)
                .format(format)
                .options(ModelOptions::default().temperature(0.0)),
        )
        .await
        .unwrap()
        .response;

    tracing::info!(response =? response);

    let judgement: Judgement = serde_json::from_str(&response).map_err(anyhow::Error::from)?;

    Ok(Json(judgement))
}
