//! Dynamic connection pool initializers for PostgreSQL (SeaORM) and Redis.

use crate::config::AppConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use shared::AppError;
use std::time::Duration;
use tracing::info;

/// Initializes PostgreSQL connection pool with SeaORM using settings from `AppConfig`
pub async fn init_db_pool(config: &AppConfig) -> Result<DatabaseConnection, AppError> {
    info!(
        "Connecting to PostgreSQL database at: {} (max_conn: {}, min_conn: {})",
        config.database.url, config.database.max_connections, config.database.min_connections
    );

    let mut opt = ConnectOptions::new(&config.database.url);
    opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(config.database.connect_timeout_secs))
        .idle_timeout(Duration::from_secs(config.database.idle_timeout_secs))
        .sqlx_logging(false);

    Database::connect(opt)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to connect to database: {e}")))
}

/// Initializes Redis connection client using settings from `AppConfig`
pub fn init_redis_client(config: &AppConfig) -> Result<redis::Client, AppError> {
    info!("Connecting to Redis at: {}", config.redis.url);

    redis::Client::open(config.redis.url.as_str())
        .map_err(|e| AppError::Internal(format!("Failed to initialize Redis client: {e}")))
}
