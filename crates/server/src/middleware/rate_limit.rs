//! Redis-backed sliding-window rate limiting middleware for public endpoints.

use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use redis::AsyncCommands;
use shared::ApiResponse;
use std::sync::Arc;

/// Limits requests on public authentication endpoints (maximum 20 requests per minute per IP)
pub async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let redis_key = format!("rate_limit:auth:{client_ip}");

    if let Ok(mut redis_conn) = state.redis.get_multiplexed_tokio_connection().await {
        let count: Result<i64, _> = redis_conn.incr(&redis_key, 1).await;
        if let Ok(c) = count {
            if c == 1 {
                let _: Result<(), _> = redis_conn.expire(&redis_key, 60).await;
            }
            if c > 20 {
                return (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(ApiResponse::<()>::error(
                        "RATE_LIMITED",
                        "Rate limit exceeded. Maximum 20 requests per minute allowed.",
                        None,
                    )),
                )
                    .into_response();
            }
        }
    }

    next.run(req).await
}
