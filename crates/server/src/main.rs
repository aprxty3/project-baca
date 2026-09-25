//! Project Baca — HTTP REST API Server
//! Built with Axum 0.8, SeaORM, and Redis.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use infra::{init_db_pool, init_redis_client, AppConfig};
use sea_orm::DatabaseConnection;
use shared::ApiResponse;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: redis::Client,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,infra=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Project Baca API Server...");

    let config = AppConfig::from_env().map_err(|e| {
        eprintln!("Configuration initialization failed: {e}");
        e
    })?;

    // Attempt DB and Redis initialization (graceful fallback if offline during initial build check)
    let db = match init_db_pool(&config).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::warn!("Postgres connection failed ({e}). Running in standalone mode.");
            DatabaseConnection::Disconnected
        }
    };

    let redis = match init_redis_client(&config) {
        Ok(client) => client,
        Err(e) => {
            tracing::warn!("Redis connection failed ({e}).");
            redis::Client::open("redis://127.0.0.1:6380")?
        }
    };

    let state = Arc::new(AppState { db, redis, config: config.clone() });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/health", get(api_health_check))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "service": "project-baca-server",
            "status": "healthy",
            "version": "0.1.0"
        }))),
    )
}

async fn api_health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
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

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!("Failed to install CTRL+C signal handler: {err}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(err) => {
                tracing::error!("Failed to install SIGTERM signal handler: {err}");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Signal received, starting graceful shutdown...");
}
