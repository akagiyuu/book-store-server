use utoipa::OpenApi;

use crate::controller;

#[derive(OpenApi)]
#[openapi(
    paths(controller::ping),
    components(schemas(crate::error::ErrorResponse,))
)]
pub struct ApiDoc;
