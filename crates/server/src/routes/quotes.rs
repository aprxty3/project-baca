//! Semantic quote search and saved-quotes management handlers.
//!
//! Endpoints:
//! - POST /api/v1/books/{book_id}/quotes/search  — Scoped HNSW cosine-distance search (SRS 17)
//! - POST /api/v1/quotes/save                    — Save a favorite quote (SRS 20, optional auth)
//! - GET  /api/v1/quotes                         — List user saved quotes (SRS 20, auth required)

use crate::{error::HttpError, middleware::AuthUser, AppState};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use infra::{
    list_saved_quotes as repo_list_saved_quotes, save_quote as repo_save_quote,
    search_quotes_by_embedding, verify_access_token,
};
use shared::{ApiResponse, AppError, QuoteSearchRequest, QuoteSearchResultDto, SaveQuoteRequest, SavedQuoteResponseDto};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

// ---------------------------------------------------------------------------
// Router builders (called from server/src/lib.rs create_app)
// ---------------------------------------------------------------------------

/// Sub-router for scoped quote search, mounted at /api/v1/books and /api/books.
/// Rate limiting is applied externally via `ai_rate_limit_middleware`.
pub fn quotes_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{book_id}/quotes/search", post(search_book_quotes))
}

/// Sub-router for saved quotes CRUD, mounted at /api/v1/quotes and /api/quotes.
pub fn saved_quotes_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/save", post(handle_save_quote))
        .route("/", get(handle_list_saved_quotes))
}

// ---------------------------------------------------------------------------
// Sub-Task 4.2: Scoped Semantic Quote Finder
// ---------------------------------------------------------------------------

/// Search for semantically relevant quotes within a specific book using HNSW cosine distance.
///
/// The query text is embedded via the configured `EmbeddingProvider` (Gemini or FastEmbed),
/// and the resulting 768-dimensional vector is matched against `book_chunks.embedding` using
/// the pgvector `<=>` operator scoped to `book_id`. Returns up to 5 results by default
/// (max 20), ordered by descending cosine similarity.
#[utoipa::path(
    post,
    path = "/api/v1/books/{book_id}/quotes/search",
    params(
        ("book_id" = Uuid, Path, description = "UUID of the book to search within")
    ),
    request_body = QuoteSearchRequest,
    responses(
        (status = 200, description = "Semantically relevant quotes ordered by cosine similarity", body = Vec<QuoteSearchResultDto>),
        (status = 400, description = "Validation error on query payload"),
        (status = 404, description = "Book not found"),
        (status = 429, description = "AI rate limit exceeded (10 req/min per user)"),
        (status = 500, description = "Embedding provider or database error")
    ),
    security(("BearerAuth" = [])),
    tag = "Semantic Search"
)]
pub async fn search_book_quotes(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthUser,
    Path(book_id): Path<Uuid>,
    Json(payload): Json<QuoteSearchRequest>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError(AppError::ValidationError(e.to_string())))?;

    let limit = payload.limit.unwrap_or(5);

    let embedding = state
        .embedding
        .embed_text(&payload.query)
        .await
        .map_err(HttpError)?;

    let rows = search_quotes_by_embedding(&state.db, book_id, &embedding, limit)
        .await
        .map_err(HttpError)?;

    let results: Vec<QuoteSearchResultDto> = rows
        .into_iter()
        .map(|r| QuoteSearchResultDto {
            chunk_id: r.chunk_id,
            chapter_number: r.chapter_number,
            chapter_title: r.chapter_title,
            content: r.content,
            similarity_score: r.similarity_score,
        })
        .collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(results)),
    ))
}

// ---------------------------------------------------------------------------
// Sub-Task 4.5: Saved Quotes Management
// ---------------------------------------------------------------------------

/// Save a favorite quote to the authenticated user's collection.
///
/// If the user is not authenticated, the request is accepted but the quote is NOT
/// persisted (guest-mode: the client is responsible for local storage). This mirrors
/// the guest-mode pattern established in the progress endpoints.
#[utoipa::path(
    post,
    path = "/api/v1/quotes/save",
    request_body = SaveQuoteRequest,
    responses(
        (status = 201, description = "Quote saved successfully"),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Database error")
    ),
    security(("BearerAuth" = [])),
    tag = "Semantic Search"
)]
pub async fn handle_save_quote(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SaveQuoteRequest>,
) -> Result<impl IntoResponse, HttpError> {
    payload
        .validate()
        .map_err(|e| HttpError(AppError::ValidationError(e.to_string())))?;

    // Attempt to extract user ID from JWT. On any failure, treat as guest.
    let maybe_user_id: Option<Uuid> = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .and_then(|token| {
            verify_access_token(token, state.config.jwt_secret())
                .ok()
                .map(|c| c.sub)
        });

    // Guest mode: accept but do not persist (return 201 for client compatibility)
    let Some(user_id) = maybe_user_id else {
        return Ok((
            StatusCode::CREATED,
            Json(ApiResponse::success(serde_json::json!({
                "saved": false,
                "reason": "guest_mode"
            }))),
        ));
    };

    let id = Uuid::new_v4();
    repo_save_quote(
        &state.db,
        id,
        user_id,
        payload.book_id,
        payload.chapter_id,
        &payload.quote_text,
    )
    .await
    .map_err(HttpError)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(serde_json::json!({ "id": id, "saved": true }))),
    ))
}

/// List all saved quotes for the authenticated user, ordered by most recent first.
#[utoipa::path(
    get,
    path = "/api/v1/quotes",
    responses(
        (status = 200, description = "List of saved quotes ordered by recency", body = Vec<SavedQuoteResponseDto>),
        (status = 401, description = "Authentication required"),
        (status = 500, description = "Database error")
    ),
    security(("BearerAuth" = [])),
    tag = "Semantic Search"
)]
pub async fn handle_list_saved_quotes(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, HttpError> {
    let quotes = repo_list_saved_quotes(&state.db, auth_user.id)
        .await
        .map_err(HttpError)?;

    let dtos: Vec<SavedQuoteResponseDto> = quotes
        .into_iter()
        .map(|q| SavedQuoteResponseDto {
            id: q.id,
            book_id: q.book_id,
            chapter_id: q.chapter_id,
            quote_text: q.quote_text,
            image_card_url: q.image_card_url,
            created_at: q.created_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(ApiResponse::success(dtos))))
}
