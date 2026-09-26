//! Project Baca — HTTP REST API Server
//! Built with Axum 0.8, SeaORM, Redis, and OpenAPI (utoipa).

use infra::{init_db_pool, init_redis_client, AppConfig};
use sea_orm::DatabaseConnection;
use server::{create_app, AppState};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "text".to_string());
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "server=debug,infra=debug,tower_http=debug".into());

    if log_format.eq_ignore_ascii_case("json") {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

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
    let app = create_app(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on http://{}", addr);
    info!("OpenAPI Swagger UI available on http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
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

