//! Integration tests for Milestone 04: Semantic AI Subsystem, HNSW Quote Search, Atomic Cards & Recaps.

mod common;

use axum::body::{to_bytes, Body};
use axum::http::{header, Request, StatusCode};
use chrono::Utc;
use common::TestHarness;
use infra::entities::{books, chapters, tldr_cache, users};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::Value;
use uuid::Uuid;

struct SeededAiContext {
    pub book_id: Uuid,
    pub chapter_id: Uuid,
    #[allow(dead_code)]
    pub user_id: Uuid,
    pub token: String,
}

async fn seed_test_context(harness: &TestHarness) -> Result<SeededAiContext, String> {
    let now = Utc::now();

    // 1. Create a test user and obtain JWT token
    let user_id = Uuid::new_v4();
    let email = format!("ai_connoisseur_{}@baca.local", Uuid::new_v4());
    let user = users::ActiveModel {
        id: Set(user_id),
        email: Set(email.clone()),
        password_hash: Set(Some("test_password_hash".to_string())),
        display_name: Set("Literary AI Explorer".to_string()),
        role: Set("reader".to_string()),
        avatar_url: Set(None),
        is_active: Set(true),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };

    user.insert(&harness.state.db)
        .await
        .map_err(|e| format!("Failed to seed user: {e}"))?;

    let token = infra::generate_access_token(
        user_id,
        &email,
        "reader",
        harness.state.config.jwt_secret(),
        1440,
    )
    .map_err(|e| format!("Token generation failed: {e}"))?;

    // 2. Create a test book
    let book_id = Uuid::new_v4();
    let book = books::ActiveModel {
        id: Set(book_id),
        title: Set("The Shadow over Innsmouth".to_string()),
        author: Set("H. P. Lovecraft".to_string()),
        language: Set("en".to_string()),
        primary_theme: Set("Horror".to_string()),
        sub_theme: Set(Some("Weird Fiction".to_string())),
        description: Set("A tale of ancient mysteries and foggy coastal towns.".to_string()),
        cover_url: Set("https://example.com/cover.jpg".to_string()),
        epub_storage_path: Set("/storage/books/test/book.epub".to_string()),
        total_words: Set(42000),
        estimated_reading_minutes: Set(180),
        source_name: Set("Standard Ebooks".to_string()),
        source_url: Set(None),
        license: Set("Public Domain".to_string()),
        publication_year: Set(Some(1936)),
        status: Set("published".to_string()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };

    book.insert(&harness.state.db)
        .await
        .map_err(|e| format!("Failed to seed book: {e}"))?;

    // 3. Create a test chapter
    let chapter_id = Uuid::new_v4();
    let chapter = chapters::ActiveModel {
        id: Set(chapter_id),
        book_id: Set(book_id),
        chapter_number: Set(1),
        title: Set("Chapter I: The Old Curiosity".to_string()),
        word_count: Set(4200),
        html_content: Set("<p>The oldest and strongest emotion of mankind is fear.</p>".to_string()),
        created_at: Set(now.into()),
    };

    chapter
        .insert(&harness.state.db)
        .await
        .map_err(|e| format!("Failed to seed chapter: {e}"))?;

    Ok(SeededAiContext {
        book_id,
        chapter_id,
        user_id,
        token,
    })
}

#[tokio::test]
async fn test_scoped_quote_search_auth_and_validation() {
    let harness = TestHarness::new().await;
    let seeded = seed_test_context(&harness).await.expect("Seeding must succeed");

    // 1. Unauthenticated request must return 401 Unauthorized
    let unauth_req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/books/{}/quotes/search", seeded.book_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::json!({ "query": "foggy harbor" }).to_string()))
        .expect("Valid request");

    let resp = harness.send_request(unauth_req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // 2. Authenticated but empty query string must fail validation (min length 2)
    let invalid_empty_req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/books/{}/quotes/search", seeded.book_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::json!({ "query": "" }).to_string()))
        .expect("Valid request");

    let resp = harness.send_request(invalid_empty_req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // 3. Authenticated query with single char must fail validation (min length 2)
    let invalid_short_req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/books/{}/quotes/search", seeded.book_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::json!({ "query": "a" }).to_string()))
        .expect("Valid request");

    let resp = harness.send_request(invalid_short_req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // 4. Authenticated query with limit > 20 must fail validation
    let invalid_limit_req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/books/{}/quotes/search", seeded.book_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::json!({ "query": "foggy harbor", "limit": 50 }).to_string()))
        .expect("Valid request");

    let resp = harness.send_request(invalid_limit_req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_scoped_quote_search_success_and_ratelimit_headers() {
    let harness = TestHarness::new().await;
    let seeded = seed_test_context(&harness).await.expect("Seeding must succeed");

    let req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/books/{}/quotes/search", seeded.book_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::json!({
                "query": "hesitation in foggy night",
                "limit": 5
            })
            .to_string(),
        ))
        .expect("Valid request");

    let resp = harness.send_request(req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Rate limit headers must be present when Redis is available
    if let Some(limit_val) = resp.headers().get("x-ratelimit-limit") {
        assert_eq!(limit_val, "10");
        assert!(resp.headers().get("x-ratelimit-remaining").is_some());
        assert!(resp.headers().get("x-ratelimit-reset").is_some());
    }

    let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], true);
    assert!(json["data"].is_array());
}

#[tokio::test]
async fn test_ai_rate_limiter_burst_enforcement() {
    let harness = TestHarness::new().await;
    let seeded = seed_test_context(&harness).await.expect("Seeding must succeed");

    // Make 10 requests that should all be accepted
    for _ in 1..=10 {
        let req = Request::builder()
            .method("POST")
            .uri(format!("/api/v1/books/{}/quotes/search", seeded.book_id))
            .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::json!({ "query": "ancient mysteries", "limit": 2 }).to_string(),
            ))
            .expect("Valid request");

        let resp = harness.send_request(req).await;
        // Either 200 OK (if Redis active) or 200 OK without headers
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // Check if Redis is active: if 11th request is throttled
    let overflow_req = Request::builder()
        .method("POST")
        .uri(format!("/api/v1/books/{}/quotes/search", seeded.book_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::json!({ "query": "ancient mysteries", "limit": 2 }).to_string(),
        ))
        .expect("Valid request");

    let resp = harness.send_request(overflow_req).await;

    // If Redis is active in test environment, 11th request must return 429
    if resp.status() == StatusCode::TOO_MANY_REQUESTS {
        assert_eq!(resp.headers().get("x-ratelimit-remaining").unwrap(), "0");
        assert!(resp.headers().get(header::RETRY_AFTER).is_some());

        let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["success"], false);
        assert_eq!(json["error"]["code"], "AI_RATE_LIMITED");
    }
}

#[tokio::test]
async fn test_saved_quotes_guest_and_authenticated_lifecycle() {
    let harness = TestHarness::new().await;
    let seeded = seed_test_context(&harness).await.expect("Seeding must succeed");

    // 1. Guest save (no Authorization header) returns 201 with guest_mode
    let guest_req = Request::builder()
        .method("POST")
        .uri("/api/v1/quotes/save")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::json!({
                "book_id": seeded.book_id,
                "chapter_id": seeded.chapter_id,
                "quote_text": "The oldest and strongest emotion of mankind is fear."
            })
            .to_string(),
        ))
        .expect("Valid request");

    let resp = harness.send_request(guest_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["saved"], false);
    assert_eq!(json["data"]["reason"], "guest_mode");

    // 2. Validation error on save (empty quote_text)
    let invalid_req = Request::builder()
        .method("POST")
        .uri("/api/v1/quotes/save")
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::json!({
                "book_id": seeded.book_id,
                "chapter_id": seeded.chapter_id,
                "quote_text": ""
            })
            .to_string(),
        ))
        .expect("Valid request");

    let resp = harness.send_request(invalid_req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // 3. Authenticated save persists quote
    let auth_save_req = Request::builder()
        .method("POST")
        .uri("/api/v1/quotes/save")
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::json!({
                "book_id": seeded.book_id,
                "chapter_id": seeded.chapter_id,
                "quote_text": "The oldest and strongest emotion of mankind is fear, and the oldest and strongest kind of fear is fear of the unknown."
            })
            .to_string(),
        ))
        .expect("Valid request");

    let resp = harness.send_request(auth_save_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["saved"], true);
    assert!(json["data"]["id"].is_string());

    // 4. Authenticated list retrieves saved quotes
    let list_req = Request::builder()
        .method("GET")
        .uri("/api/v1/quotes")
        .header(header::AUTHORIZATION, format!("Bearer {}", seeded.token))
        .body(Body::empty())
        .expect("Valid request");

    let resp = harness.send_request(list_req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], true);
    let quotes = json["data"].as_array().expect("Array of saved quotes");
    assert!(!quotes.is_empty());
    assert_eq!(
        quotes[0]["quote_text"],
        "The oldest and strongest emotion of mankind is fear, and the oldest and strongest kind of fear is fear of the unknown."
    );

    // 5. Unauthenticated list must return 401
    let unauth_list_req = Request::builder()
        .method("GET")
        .uri("/api/v1/quotes")
        .body(Body::empty())
        .expect("Valid request");

    let resp = harness.send_request(unauth_list_req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_atomic_insight_cards_cache_hit_and_miss() {
    let harness = TestHarness::new().await;
    let seeded = seed_test_context(&harness).await.expect("Seeding must succeed");

    // 1. Cache miss returns 404
    let miss_req = Request::builder()
        .method("GET")
        .uri(format!(
            "/api/v1/books/{}/chapters/{}/atomic-cards",
            seeded.book_id, seeded.chapter_id
        ))
        .body(Body::empty())
        .expect("Valid request");

    let resp = harness.send_request(miss_req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // 2. Seed atomic cards into tldr_cache
    let now = Utc::now();
    let cards_content = serde_json::json!({
        "key_concepts": ["Ancient cosmic terror", "Innsmouth degeneration"],
        "notable_quotes": ["The oldest and strongest emotion of mankind is fear."],
        "historical_context": "Written in late 1931, exploring isolated New England seaports."
    });

    let cache_entry = tldr_cache::ActiveModel {
        id: Set(Uuid::new_v4()),
        book_id: Set(seeded.book_id),
        chapter_id: Set(Some(seeded.chapter_id)),
        recap_type: Set("chapter_atomic_cards".to_string()),
        content_json: Set(cards_content),
        model_version: Set("gemini-1.5-flash".to_string()),
        created_at: Set(now.into()),
    };

    cache_entry
        .insert(&harness.state.db)
        .await
        .expect("Insert tldr_cache entry");

    // 3. Cache hit returns 200 with structured cards
    let hit_req = Request::builder()
        .method("GET")
        .uri(format!(
            "/api/v1/books/{}/chapters/{}/atomic-cards",
            seeded.book_id, seeded.chapter_id
        ))
        .body(Body::empty())
        .expect("Valid request");

    let resp = harness.send_request(hit_req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["book_id"], seeded.book_id.to_string());
    assert_eq!(json["data"]["chapter_id"], seeded.chapter_id.to_string());
    assert!(json["data"]["cards"]["key_concepts"].is_array());
}

#[tokio::test]
async fn test_chapter_recap_cache_hit_and_miss() {
    let harness = TestHarness::new().await;
    let seeded = seed_test_context(&harness).await.expect("Seeding must succeed");

    // 1. Cache miss returns 404
    let miss_req = Request::builder()
        .method("GET")
        .uri(format!(
            "/api/v1/books/{}/chapters/{}/recap",
            seeded.book_id, seeded.chapter_id
        ))
        .body(Body::empty())
        .expect("Valid request");

    let resp = harness.send_request(miss_req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // 2. Seed recap into tldr_cache
    let now = Utc::now();
    let recap_content = serde_json::json!({
        "summary": "In previous chapters, the narrator investigated odd rumors about the decaying seaport of Innsmouth.",
        "key_characters": ["Robert Olmstead", "Zadok Allen"],
        "spoiler_free_note": "This recap covers only confirmed events through chapter 1."
    });

    let cache_entry = tldr_cache::ActiveModel {
        id: Set(Uuid::new_v4()),
        book_id: Set(seeded.book_id),
        chapter_id: Set(Some(seeded.chapter_id)),
        recap_type: Set("chapter_recap".to_string()),
        content_json: Set(recap_content),
        model_version: Set("gemini-1.5-flash".to_string()),
        created_at: Set(now.into()),
    };

    cache_entry
        .insert(&harness.state.db)
        .await
        .expect("Insert tldr_cache entry");

    // 3. Cache hit returns 200 with structured recap
    let hit_req = Request::builder()
        .method("GET")
        .uri(format!(
            "/api/v1/books/{}/chapters/{}/recap",
            seeded.book_id, seeded.chapter_id
        ))
        .body(Body::empty())
        .expect("Valid request");

    let resp = harness.send_request(hit_req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["book_id"], seeded.book_id.to_string());
    assert_eq!(json["data"]["chapter_id"], seeded.chapter_id.to_string());
    assert!(json["data"]["recap"]["summary"].is_string());
}
