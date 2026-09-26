//! Per-user AI rate limit middleware for semantic search endpoints.
//!
//! Enforces a sliding-window limit of 10 requests per minute per authenticated user.
//! If the Authorization header is absent or invalid, the request falls through without
//! rate limiting (the handler itself will reject it via the `AuthUser` extractor if auth
//! is required, so there is no security gap here).
//!
//! Redis key pattern: `rate_limit:ai:<user_id>` (TTL: 60 seconds).

use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderName, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use infra::verify_access_token;
use redis::AsyncCommands;
use shared::ApiResponse;
use std::sync::Arc;

const AI_MAX_REQUESTS: i64 = 10;
const AI_WINDOW_SECONDS: i64 = 60;

/// Sliding-window rate limiter scoped to the authenticated user identity.
///
/// Reads the JWT from the `Authorization: Bearer <token>` header without
/// performing full token validation (no Redis blacklist check here — that is
/// deferred to the `AuthUser` extractor in each handler). This keeps
/// middleware overhead minimal.
pub async fn ai_rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    // Extract user ID from JWT claim without full blacklist validation.
    // On any error, pass through (the handler's AuthUser extractor will enforce auth).
    let maybe_user_id: Option<String> = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .and_then(|token| {
            verify_access_token(token, state.config.jwt_secret())
                .ok()
                .map(|c| c.sub.to_string())
        });

    let Some(user_id) = maybe_user_id else {
        return next.run(req).await;
    };

    let redis_key = format!("rate_limit:ai:{user_id}");

    if let Ok(mut redis_conn) = state.get_redis_conn().await {
        let count: Result<i64, _> = redis_conn.incr(&redis_key, 1).await;
        if let Ok(c) = count {
            let mut ttl: i64 = redis_conn.ttl(&redis_key).await.unwrap_or(-1);
            if ttl <= 0 {
                let _: Result<(), _> = redis_conn.expire(&redis_key, AI_WINDOW_SECONDS).await;
                ttl = AI_WINDOW_SECONDS;
            }

            let reset_seconds = if ttl > 0 { ttl } else { AI_WINDOW_SECONDS };
            let remaining = (AI_MAX_REQUESTS - c).max(0);

            if c > AI_MAX_REQUESTS {
                let mut resp = (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(ApiResponse::<()>::error(
                        "AI_RATE_LIMITED",
                        &format!(
                            "AI rate limit exceeded. Maximum {AI_MAX_REQUESTS} semantic \
                             search requests per minute allowed."
                        ),
                        None,
                    )),
                )
                    .into_response();

                let headers = resp.headers_mut();
                if let Ok(v) = HeaderValue::from_str(&AI_MAX_REQUESTS.to_string()) {
                    headers.insert(HeaderName::from_static("x-ratelimit-limit"), v);
                }
                headers.insert(
                    HeaderName::from_static("x-ratelimit-remaining"),
                    HeaderValue::from_static("0"),
                );
                if let Ok(v) = HeaderValue::from_str(&reset_seconds.to_string()) {
                    headers.insert(HeaderName::from_static("x-ratelimit-reset"), v.clone());
                    headers.insert(header::RETRY_AFTER, v);
                }
                return resp;
            }

            let mut response = next.run(req).await;
            let headers = response.headers_mut();
            if let Ok(v) = HeaderValue::from_str(&AI_MAX_REQUESTS.to_string()) {
                headers.insert(HeaderName::from_static("x-ratelimit-limit"), v);
            }
            if let Ok(v) = HeaderValue::from_str(&remaining.to_string()) {
                headers.insert(HeaderName::from_static("x-ratelimit-remaining"), v);
            }
            if let Ok(v) = HeaderValue::from_str(&reset_seconds.to_string()) {
                headers.insert(HeaderName::from_static("x-ratelimit-reset"), v);
            }
            return response;
        }
    }

    next.run(req).await
}
