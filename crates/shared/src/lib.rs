//! Shared DTOs, Error models, and validation logic for Project Baca.
//! This crate is utilized by both backend Axum and frontend Leptos WASM.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorPayload>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: &str, message: &str, details: Option<serde_json::Value>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorPayload {
                code: code.to_string(),
                message: message.to_string(),
                details,
            }),
        }
    }
}

/// Standardized error payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ErrorPayload {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Common application errors
#[derive(Debug, thiserror::Error, Serialize, Deserialize, Clone)]
pub enum AppError {
    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limit exceeded. Try again in {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("External service error: {0}")]
    ExternalService(String),
}

// -----------------------------------------------------------------------------
// Authentication DTOs
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SignupRequest {
    #[validate(length(min = 2, max = 50, message = "Nickname must be between 2 and 50 characters"))]
    pub nickname: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, max = 128, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub re_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VerifyOtpRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(equal = 6, message = "OTP must be 6 digits"))]
    pub otp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UserProfileDto {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// Catalog & Book DTOs
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct BookSummaryDto {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub author: String,
    pub cover_url: Option<String>,
    pub word_count: i32,
    pub reading_time_minutes: i32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct BookDetailDto {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub author: String,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub word_count: i32,
    pub reading_time_minutes: i32,
    pub tags: Vec<String>,
    pub chapters: Vec<ChapterSummaryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ChapterSummaryDto {
    pub id: Uuid,
    pub chapter_number: i32,
    pub title: String,
    pub word_count: i32,
}

// -----------------------------------------------------------------------------
// Semantic & Reading DTOs
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct QuoteSearchRequest {
    #[validate(length(min = 3, max = 500, message = "Query must be between 3 and 500 characters"))]
    pub query: String,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct QuoteSearchResultDto {
    pub chunk_id: Uuid,
    pub chapter_number: i32,
    pub content: String,
    pub similarity_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ReadingProgressUpdateDto {
    pub chapter_id: Uuid,
    pub cfi_position: String,
    pub percentage: f32,
}
