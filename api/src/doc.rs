use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(), components(schemas(crate::error::ErrorResponse,)))]
pub struct ApiDoc;
