mod config;
mod controller;
mod doc;
mod error;
mod state;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    http::{
        HeaderName, HeaderValue, Method,
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
        },
    },
    routing::get,
};
use config::CONFIG;
use doc::ApiDoc;
use error::Error;
use state::ApiState;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_swagger_ui::SwaggerUi;

type Result<T> = std::result::Result<T, Error>;

const ALLOW_HEADERS: [HeaderName; 7] = [
    ORIGIN,
    AUTHORIZATION,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
    ACCEPT,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_HEADERS,
];
const ALLOW_METHODS: [Method; 5] = [
    Method::GET,
    Method::POST,
    Method::DELETE,
    Method::PATCH,
    Method::PUT,
];

fn build(state: Arc<ApiState>) -> Router {
    let allow_origins = [CONFIG.cors_domain.parse::<HeaderValue>().unwrap()];

    let router = Router::new().route("/", get(controller::ping));

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let router = router.layer(TraceLayer::new_for_http()).layer(
        CorsLayer::new()
            .allow_origin(allow_origins)
            .allow_headers(ALLOW_HEADERS)
            .expose_headers(ALLOW_HEADERS)
            .allow_credentials(true)
            .allow_methods(ALLOW_METHODS),
    );

    router.with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_timer(fmt::time::ChronoLocal::rfc_3339()),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    let state = Arc::new(ApiState::new().await.unwrap());
    let app = build(state);
    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), CONFIG.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
