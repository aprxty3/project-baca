//! HTTP Error mapping for Project Baca Axum REST API.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared::{ApiResponse, AppError};

/// Wrapper converting internal `AppError` into standardized Axum HTTP responses
#[derive(Debug)]
pub struct HttpError(pub AppError);

impl From<AppError> for HttpError {
    fn from(err: AppError) -> Self {
        HttpError(err)
    }
}

impl From<sea_orm::DbErr> for HttpError {
    fn from(err: sea_orm::DbErr) -> Self {
        HttpError(AppError::Internal(format!("Database error: {err}")))
    }
}

impl From<validator::ValidationErrors> for HttpError {
    fn from(err: validator::ValidationErrors) -> Self {
        HttpError(AppError::ValidationError(format!(
            "Validation failed: {err}"
        )))
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self.0 {
            AppError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, "VALIDATION_FAILED", msg.clone())
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::RateLimited { retry_after } => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMITED",
                format!("Rate limit exceeded. Try again in {retry_after} seconds"),
            ),
            AppError::Database(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                msg.clone(),
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg.clone(),
            ),
            AppError::ExternalService(msg) => (
                StatusCode::BAD_GATEWAY,
                "EXTERNAL_SERVICE_ERROR",
                msg.clone(),
            ),
        };

        (status, Json(ApiResponse::<()>::error(code, &message, None))).into_response()
    }
}
