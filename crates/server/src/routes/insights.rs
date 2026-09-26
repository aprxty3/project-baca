//! Atomic insight cards and spoiler-free chapter recap handlers.
//!
//! Endpoints:
//! - GET /api/v1/books/{book_id}/chapters/{chapter_id}/atomic-cards  — Chapter atomic insight cards (SRS 18)
//! - GET /api/v1/books/{book_id}/chapters/{chapter_id}/recap         — Spoiler-free catch-up recap (SRS 19)
//!
//! Both endpoints read from the `tldr_cache` table with sub-5ms latency on cache hit.
//! On cache miss, 404 is returned. Actual cache population is handled by the Python
//! ingestion worker (Task 05).

use crate::{error::HttpError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use infra::{get_chapter_recap as query_chapter_recap, get_tldr_cache};
use shared::{ApiResponse, AppError, AtomicCardsDto, ChapterRecapDto};
use std::sync::Arc;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Router builder
// ---------------------------------------------------------------------------

/// Sub-router for AI insight endpoints, mounted at /api/v1/books and /api/books.
/// These endpoints are public-read (no auth required for cache-hit reads).
pub fn insights_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/{book_id}/chapters/{chapter_id}/atomic-cards",
            get(get_atomic_cards),
        )
        .route(
            "/{book_id}/chapters/{chapter_id}/recap",
            get(get_chapter_recap),
        )
}

// ---------------------------------------------------------------------------
// Sub-Task 4.3: Chapter Atomic Insight Cards (SRS 18)
// ---------------------------------------------------------------------------

/// Retrieve precomputed atomic insight cards for a chapter from `tldr_cache`.
///
/// Returns structured JSON with categories: `key_concept`, `notable_quote`,
/// `historical_context`. Zero token cost on cache hit (sub-5ms).
/// Returns 404 if the cache entry has not been populated yet by the ingestion worker.
#[utoipa::path(
    get,
    path = "/api/v1/books/{book_id}/chapters/{chapter_id}/atomic-cards",
    params(
        ("book_id" = Uuid, Path, description = "UUID of the book"),
        ("chapter_id" = Uuid, Path, description = "UUID of the chapter")
    ),
    responses(
        (status = 200, description = "Atomic insight cards retrieved from cache", body = AtomicCardsDto),
        (status = 404, description = "Atomic cards not yet generated for this chapter"),
        (status = 500, description = "Database error")
    ),
    tag = "Semantic Search"
)]
pub async fn get_atomic_cards(
    State(state): State<Arc<AppState>>,
    Path((book_id, chapter_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, HttpError> {
    let maybe_cards =
        get_tldr_cache(&state.db, book_id, chapter_id, "chapter_atomic_cards")
            .await
            .map_err(HttpError)?;

    match maybe_cards {
        Some(cards_json) => {
            let dto = AtomicCardsDto {
                book_id,
                chapter_id,
                cards: cards_json,
            };
            Ok((StatusCode::OK, Json(ApiResponse::success(dto))))
        }
        None => Err(HttpError(AppError::NotFound(format!(
            "Atomic cards for chapter {chapter_id} have not been generated yet."
        )))),
    }
}

// ---------------------------------------------------------------------------
// Sub-Task 4.4: Spoiler-Free Catch-up Recap (SRS 19)
// ---------------------------------------------------------------------------

/// Retrieve a spoiler-free catch-up summary for a chapter from `tldr_cache`.
///
/// Summarizes key events from prior chapters. Only meaningful for chapters > 1;
/// for chapter 1 the cache will be empty and a 404 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/books/{book_id}/chapters/{chapter_id}/recap",
    params(
        ("book_id" = Uuid, Path, description = "UUID of the book"),
        ("chapter_id" = Uuid, Path, description = "UUID of the chapter")
    ),
    responses(
        (status = 200, description = "Spoiler-free catch-up recap retrieved from cache", body = ChapterRecapDto),
        (status = 404, description = "Recap not yet generated for this chapter"),
        (status = 500, description = "Database error")
    ),
    tag = "Semantic Search"
)]
pub async fn get_chapter_recap(
    State(state): State<Arc<AppState>>,
    Path((book_id, chapter_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, HttpError> {
    let maybe_recap = query_chapter_recap(&state.db, book_id, chapter_id)
        .await
        .map_err(HttpError)?;

    match maybe_recap {
        Some(recap_json) => {
            let dto = ChapterRecapDto {
                book_id,
                chapter_id,
                recap: recap_json,
            };
            Ok((StatusCode::OK, Json(ApiResponse::success(dto))))
        }
        None => Err(HttpError(AppError::NotFound(format!(
            "Recap for chapter {chapter_id} has not been generated yet."
        )))),
    }
}
