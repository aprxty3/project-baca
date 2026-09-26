//! Project Baca — Server Library
//! Exposes Axum Router, AppState, Middleware, OpenAPI specs, and HTTP Handlers.

use axum::{
    body::Body,
    extract::State,
    http::{HeaderName, HeaderValue, Request, Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use infra::AppConfig;
use sea_orm::DatabaseConnection;
use shared::*;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub static REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: redis::Client,
    pub config: AppConfig,
}

/// OpenAPI documentation root schema
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        api_health_check
    ),
    components(
        schemas(
            ErrorPayload,
            SignupRequest,
            VerifyOtpRequest,
            LoginRequest,
            TokenResponse,
            UserProfileDto,
            BookSummaryDto,
            BookDetailDto,
            ChapterSummaryDto,
            QuoteSearchRequest,
            QuoteSearchResultDto,
            ReadingProgressUpdateDto
        )
    ),
    tags(
        (name = "System & Health", description = "Runtime health checks and infrastructure connectivity")
    ),
    info(
        title = "Project Baca REST API",
        version = "0.1.0",
        description = "Public Domain Classic E-Reader & Atomic Insights API.",
        license(name = "MIT OR Apache-2.0")
    )
)]
pub struct ApiDoc;

/// Middleware for request ID correlation (x-request-id) across tracing spans and HTTP responses
pub async fn request_id_middleware(req: Request<Body>, next: Next) -> Response<Body> {
    let request_id = match req.headers().get(&REQUEST_ID_HEADER) {
        Some(id) => id.clone(),
        None => {
            let new_id = uuid::Uuid::new_v4().to_string();
            HeaderValue::from_str(&new_id).unwrap_or_else(|_| HeaderValue::from_static("unknown"))
        }
    };

    let req_id_str = request_id.to_str().unwrap_or("unknown").to_string();
    let span = tracing::info_span!("http_request", request_id = %req_id_str);
    let _guard = span.enter();

    let mut response = next.run(req).await;
    response.headers_mut().insert(REQUEST_ID_HEADER.clone(), request_id);
    response
}

/// Basic server health check handler
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Basic server health status")
    ),
    tag = "System & Health"
)]
pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "service": "project-baca-server",
            "status": "healthy",
            "version": "0.1.0"
        }))),
    )
}

/// Infrastructure health check handler for database and cache connectivity
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Database and cache connectivity status")
    ),
    tag = "System & Health"
)]
pub async fn api_health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db_connected = state.db.ping().await.is_ok();
    
    (
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "status": "ok",
            "postgres": if db_connected { "connected" } else { "disconnected" },
            "redis": "configured",
            "port": state.config.port
        }))),
    )
}

/// Assembles Axum Router with Swagger UI and middleware pipeline
pub fn create_app(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/api/v1/health", get(api_health_check))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
