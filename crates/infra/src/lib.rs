//! Infrastructure configuration, connection pools, and database integrations.

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use shared::AppError;
use std::time::Duration;
use tracing::info;

/// Infrastructure configuration holding database, cache, and external service URLs
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub minio_endpoint: String,
    pub minio_bucket: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://baca_user:baca_password@127.0.0.1:5433/project_baca_db".to_string()
        });

        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6380".to_string());

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "project-baca-local-dev-jwt-secret-key-32chars!".to_string());

        let minio_endpoint = std::env::var("MINIO_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:9000".to_string());

        let minio_bucket = std::env::var("MINIO_BUCKET")
            .unwrap_or_else(|_| "project-baca-books".to_string());

        Ok(Self {
            database_url,
            redis_url,
            port,
            jwt_secret,
            minio_endpoint,
            minio_bucket,
        })
    }
}

/// Initialize PostgreSQL connection pool with SeaORM
pub async fn init_db_pool(config: &AppConfig) -> Result<DatabaseConnection, AppError> {
    info!("Connecting to PostgreSQL database: {}", config.database_url);

    let mut opt = ConnectOptions::new(&config.database_url);
    opt.max_connections(25)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(false);

    Database::connect(opt)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to connect to database: {e}")))
}

/// Initialize Redis connection client
pub fn init_redis_client(config: &AppConfig) -> Result<redis::Client, AppError> {
    info!("Connecting to Redis: {}", config.redis_url);

    redis::Client::open(config.redis_url.clone())
        .map_err(|e| AppError::Internal(format!("Failed to initialize Redis client: {e}")))
}
