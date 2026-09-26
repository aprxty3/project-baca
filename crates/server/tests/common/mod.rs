//! Common test utilities, test harness, and app factories for Project Baca.

use axum::body::Body;
use axum::http::{Request, Response};
use axum::Router;
use infra::{init_db_pool, init_redis_client, AppConfig};
use sea_orm::DatabaseConnection;
use server::{create_app, AppState};
use std::sync::Arc;
use tower::ServiceExt;

pub struct TestHarness {
    pub app: Router,
    #[allow(dead_code)]
    pub state: Arc<AppState>,
}

impl TestHarness {
    /// Inisialisasi test harness dengan koneksi riil jika container aktif, atau fallback graceful
    pub async fn new() -> Self {
        let config = AppConfig::from_env().unwrap_or_else(|_| AppConfig {
            database_url: "postgresql://baca_user:baca_password@127.0.0.1:5433/project_baca_db".to_string(),
            redis_url: "redis://127.0.0.1:6380".to_string(),
            port: 8080,
            jwt_secret: "test-secret-key-32-bytes-minimum!".to_string(),
            minio_endpoint: "http://127.0.0.1:9005".to_string(),
            minio_bucket: "test-bucket".to_string(),
        });

        let db = match init_db_pool(&config).await {
            Ok(pool) => pool,
            Err(_) => DatabaseConnection::Disconnected,
        };

        let redis = match init_redis_client(&config) {
            Ok(client) => client,
            Err(_) => redis::Client::open("redis://127.0.0.1:6380").unwrap(),
        };

        let state = Arc::new(AppState {
            db,
            redis,
            config,
        });

        let app = create_app(state.clone());

        Self { app, state }
    }

    /// Eksekusi request HTTP ke dalam instance router Axum in-memory
    pub async fn send_request(&self, req: Request<Body>) -> Response<Body> {
        self.app.clone().oneshot(req).await.expect("Failed to execute in-memory request")
    }

    /// Eksekusi request dan deserialisasi response body menjadi JSON
    #[allow(dead_code)]
    pub async fn send_json_request(&self, req: Request<Body>) -> (Response<Body>, serde_json::Value) {
        let resp = self.send_request(req).await;
        let (parts, body) = resp.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .expect("Failed to read response bytes");
        let json: serde_json::Value = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| serde_json::json!({ "raw": String::from_utf8_lossy(&bytes).to_string() }));
        (Response::from_parts(parts, Body::from(bytes)), json)
    }
}
