//! Reading Progress & Guest Reconciliation REST API Handlers (SRS 14–16)

use crate::{error::HttpError, middleware::auth::AuthUser, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use chrono::Utc;
use sea_orm::{entity::prelude::Decimal, ConnectionTrait, DatabaseBackend, Statement};
use shared::{ApiResponse, ErrorPayload, GuestMergeRequest};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

/// Merges guest reading progress records into the authenticated user's account (SRS 16)
#[utoipa::path(
    post,
    path = "/api/v1/progress/merge",
    request_body = GuestMergeRequest,
    responses(
        (status = 200, description = "Guest progress reconciled successfully", body = ApiResponse<serde_json::Value>),
        (status = 400, description = "Invalid payload", body = ApiResponse<ErrorPayload>),
        (status = 401, description = "Unauthorized", body = ApiResponse<ErrorPayload>)
    ),
    security(("BearerAuth" = [])),
    tag = "Reading Progress"
)]
pub async fn merge_guest_progress(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(req): Json<GuestMergeRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let mut merged_count = 0;

    for record in req.records {
        let decimal_str = format!("{:.2}", record.completion_percentage);
        let completion: Decimal = decimal_str.parse().unwrap_or(Decimal::ZERO);
        let last_read_at = record.last_read_at.unwrap_or_else(Utc::now);
        let is_finished = record.is_finished.unwrap_or(false);

        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
            INSERT INTO user_reading_progress (
                id, user_id, book_id, last_chapter_id, last_anchor_cfi,
                completion_percentage, is_finished, last_read_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            ON CONFLICT (user_id, book_id) DO UPDATE SET
                last_chapter_id = EXCLUDED.last_chapter_id,
                last_anchor_cfi = EXCLUDED.last_anchor_cfi,
                completion_percentage = GREATEST(user_reading_progress.completion_percentage, EXCLUDED.completion_percentage),
                is_finished = (user_reading_progress.is_finished OR EXCLUDED.is_finished),
                last_read_at = GREATEST(user_reading_progress.last_read_at, EXCLUDED.last_read_at),
                updated_at = NOW();
            "#,
            vec![
                Uuid::new_v4().into(),
                auth.id.into(),
                record.book_id.into(),
                record.last_chapter_id.into(),
                record.last_anchor_cfi.into(),
                completion.into(),
                is_finished.into(),
                last_read_at.into(),
            ],
        );

        state.db.execute(stmt).await.map_err(HttpError::from)?;
        merged_count += 1;
    }

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Guest progress successfully merged",
            "merged_count": merged_count
        }))),
    )
        .into_response())
}

/// Assembles reading progress routes
pub fn progress_routes() -> Router<Arc<AppState>> {
    Router::new().route("/merge", post(merge_guest_progress))
}
