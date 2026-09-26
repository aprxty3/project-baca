//! JWT Authentication extractor for Axum handlers.

use crate::{error::HttpError, AppState};
use axum::{extract::FromRequestParts, http::request::Parts};
use infra::security::jwt::verify_access_token;
use shared::AppError;
use std::sync::Arc;
use uuid::Uuid;

/// Authenticated user extracted from a valid JWT Bearer token
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = HttpError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                HttpError(AppError::Unauthorized(
                    "Missing Authorization header".to_string(),
                ))
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            HttpError(AppError::Unauthorized(
                "Invalid Authorization scheme. Bearer expected".to_string(),
            ))
        })?;

        let claims =
            verify_access_token(token, state.config.jwt_secret()).map_err(HttpError::from)?;

        Ok(AuthUser {
            id: claims.sub,
            email: claims.email,
            role: claims.role,
        })
    }
}
