# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- 103 files · ~202,417 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 6 file(s) not represented in the graph (top: (none) 3, .example 1, .toml 1)

## Summary
- 816 nodes · 1448 edges · 92 communities (62 shown, 30 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 26 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `55d9bfb6`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- reliability_test.rs
- Database Schema and Indexes
- shared/src/lib.rs
- server/src/lib.rs
- 2. Work Breakdown
- domain/src/lib.rs
- server/src/main.rs
- Frontend Web Reader Tasks
- Test Harness Utilities
- 2. Work Breakdown
- EPUB Ingestion Pipeline Tasks
- Quality Assurance and Benchmarking
- README.md
- AppError
- 2. Work Breakdown
- Database Migration Scripts
- Project Workspace Modules
- AppConfig
- prelude
- Knowledge Graph Protocols
- progress_repository.rs
- web/src/main.rs
- Reader UI Illustrations
- Catalog UI Illustrations
- Monitoring UI Illustrations
- 2. Work Breakdown
- entities/mod.rs
- AppState
- pool.rs
- smoke_test.rs
- catalog_test.rs
- 2. Step-by-Step Test Procedure
- Model
- manual-test/README.md
- Manual Testing — Milestone 03: Catalog, Reader Engine & Gamification
- 2. Documentation Directory
- Knowledge Format Specification
- Database Technology Stack
- Model
- .find_book_by_id
- 3. Server Health & OpenAPI Documentation
- Model
- serde
- Model
- Model
- Model
- chapters.rs
- reading_activity_logs.rs
- tags.rs
- Entity
- user_badges.rs
- badges.rs
- 2. Prerequisites & Service Status Verification
- 3. Catalog Discovery & Typo-Tolerant Search
- middleware/mod.rs
- Entity
- Entity
- Entity
- Entity
- Entity
- Entity
- Entity
- Entity
- Entity
- Entity
- Entity
- 4. Book Overview, Reader Content & Offline Bundle
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- ActiveModel
- testharness
- rate_limit_middleware
- Q: nah pertanyaanmu terkait graf seriusan ga paham, apalagi across architetureal boundaries gitu. tolong deh telusuri jalur data dan dependencynya.

## God Nodes (most connected - your core abstractions)
1. `AppError` - 39 edges
2. `AppState` - 37 edges
3. `HttpError` - 29 edges
4. `AppConfig` - 18 edges
5. `AuthUser` - 18 edges
6. `verify_otp()` - 14 edges
7. `login()` - 14 edges
8. `TestHarness` - 14 edges
9. `signup()` - 12 edges
10. `refresh()` - 12 edges

## Surprising Connections (you probably didn't know these)
- `Web Entry Point` --semantically_similar_to--> `Asset Catalog`  [INFERRED] [semantically similar]
  crates/web/index.html → assets/README.md
- `signup()` --calls--> `send_otp_email()`  [INFERRED]
  crates/server/src/routes/auth.rs → crates/infra/src/email.rs
- `main()` --calls--> `init_db_pool()`  [INFERRED]
  crates/server/src/main.rs → crates/infra/src/pool.rs
- `main()` --calls--> `init_redis_client()`  [INFERRED]
  crates/server/src/main.rs → crates/infra/src/pool.rs
- `login()` --calls--> `generate_access_token()`  [INFERRED]
  crates/server/src/routes/auth.rs → crates/infra/src/security/jwt.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Infrastructure Services** — redis_7 [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]
- **MVP Implementation Sequence** — knowledge_tasks_01_infrastructure_and_config, knowledge_tasks_02_authentication_and_user, knowledge_tasks_03_catalog_and_reader_backend, knowledge_tasks_04_semantic_ai_and_insights, knowledge_tasks_05_ingestion_pipeline_and_admin, knowledge_tasks_06_frontend_leptos_web_reader, knowledge_tasks_07_quality_assurance_and_launch [EXTRACTED 1.00]
- **OKF Knowledge Vault** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]

## Communities (92 total, 30 thin omitted)

### Community 0 - "reliability_test.rs"
Cohesion: 0.13
Nodes (8): appconfig, arc, assert_eq, automock, body, http, instant, uuid

### Community 1 - "Database Schema and Indexes"
Cohesion: 0.11
Nodes (29): idx_users_admin_role, idx_users_email, users, book_tags, books, idx_book_tags_reverse, idx_books_catalog_filter, idx_books_published_author_trgm (+21 more)

### Community 2 - "shared/src/lib.rs"
Cohesion: 0.10
Nodes (49): get_book_by_id(), get_chapter_by_number(), get_offline_bundle(), get_tags_for_book(), list_books(), DatabaseConnection, Result, String (+41 more)

### Community 3 - "server/src/lib.rs"
Cohesion: 0.09
Nodes (23): cors, api_health_check(), ApiDoc, create_app(), health_check(), REQUEST_ID_HEADER, request_id_middleware(), Arc (+15 more)

### Community 4 - "2. Work Breakdown"
Cohesion: 0.20
Nodes (10): 1. Summary, 2. Work Breakdown, 3. Success Criteria & Verification, Cross-Domain Matrix, Sub-Task 2.1: Password Hashing (Argon2id) & OTP [COMPLETED], Sub-Task 2.2: JWT Tokens & Refresh Rotation [COMPLETED], Sub-Task 2.3: REST API Endpoints (SRS 1–7) [COMPLETED], Sub-Task 2.4: Guest Reconciliation (`POST /api/v1/progress/merge`) [COMPLETED] (+2 more)

### Community 5 - "domain/src/lib.rs"
Cohesion: 0.28
Nodes (16): chrono, Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak (+8 more)

### Community 6 - "server/src/main.rs"
Cohesion: 0.20
Nodes (10): Box, main(), Result, shutdown_signal(), databaseconnection, Error, infra, server (+2 more)

### Community 7 - "Frontend Web Reader Tasks"
Cohesion: 0.17
Nodes (12): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 6.1: Vintage Literary Design System (1900–1950), Sub-Task 6.2: API Client & Local-First IndexedDB (`rexie`), Sub-Task 6.3: Home Page (Catalog & Search), Sub-Task 6.4: Book Overview Page (+4 more)

### Community 8 - "Test Harness Utilities"
Cohesion: 0.31
Nodes (8): Arc, Body, Request, Response, Router, Value, TestHarness, serviceext

### Community 9 - "2. Work Breakdown"
Cohesion: 0.29
Nodes (7): 2. Work Breakdown, Sub-Task 1.1: `dotenvy` Integration, Sub-Task 1.2: Modular `AppConfig` Sub-Configurations, Sub-Task 1.3: Dynamic Connection Pools, Sub-Task 1.4: SeaORM Entities for 13 Tables, Sub-Task 1.5: Server `AppState`, Sub-Task 1.6: OpenAPI & Observability

### Community 10 - "EPUB Ingestion Pipeline Tasks"
Cohesion: 0.18
Nodes (11): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 5.1: S3/MinIO Storage Service (`crates/infra`), Sub-Task 5.2: Admin EPUB Upload Endpoint (SRS 21), Sub-Task 5.3: Ingestion Worker & EPUB Parsing, Sub-Task 5.4: HTML Sanitization & Scene Chunking (+3 more)

### Community 11 - "Quality Assurance and Benchmarking"
Cohesion: 0.18
Nodes (11): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 7.1: Smoke Testing Suite, Sub-Task 7.2: Unit Test Suite, Sub-Task 7.3: Integration Test Suite, Sub-Task 7.4: Performance SLA Benchmarks (+3 more)

### Community 12 - "README.md"
Cohesion: 0.15
Nodes (16): OKF & Persistent Memory Guidelines, Admin Sorting Illustration, AI Quote Finder Illustration, Asset Catalog, GBrain Design Reference, Web Entry Point, 1. Summary, 3. Success Criteria (+8 more)

### Community 13 - "AppError"
Cohesion: 0.09
Nodes (40): argon2, Claims, generate_access_token(), generate_refresh_token(), revoke_refresh_token(), MultiplexedConnection, Result, String (+32 more)

### Community 14 - "2. Work Breakdown"
Cohesion: 0.22
Nodes (9): 1. Summary, 2. Work Breakdown, 3. Success Criteria & Verification, Cross-Domain Matrix, Sub-Task 3.1: Catalog Repository & FTS Lexical Search [COMPLETED], Sub-Task 3.2: Catalog & Chapter Endpoints (SRS 8–12) [COMPLETED], Sub-Task 3.3: Reading Progress & CFI Synchronization (SRS 13–14) [COMPLETED], Sub-Task 3.4: Reading Heartbeat, Streaks & Badges (SRS 15–16) [COMPLETED] (+1 more)

### Community 15 - "Database Migration Scripts"
Cohesion: 0.76
Nodes (6): init_table(), migrate_down(), migrate_status(), migrate_up(), psql_cmd(), migrate.sh script

### Community 16 - "Project Workspace Modules"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 17 - "AppConfig"
Cohesion: 0.12
Nodes (19): AiConfig, AppConfig, AuthConfig, DatabaseConfig, EmailConfig, RedisConfig, Result, Self (+11 more)

### Community 18 - "prelude"
Cohesion: 0.33
Nodes (4): Relation, App(), IntoView, prelude

### Community 20 - "progress_repository.rs"
Cohesion: 0.14
Nodes (24): Model, DateTimeWithTimeZone, String, Uuid, list_badges(), list_user_badges(), DatabaseConnection, Result (+16 more)

### Community 25 - "2. Work Breakdown"
Cohesion: 0.20
Nodes (10): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 4.1: Dual-Mode Embedding Provider (`crates/infra`), Sub-Task 4.2: Scoped Semantic Quote Finder (`POST /api/books/{id}/quotes/search`), Sub-Task 4.3: Chapter Atomic Insight Cards (SRS 18), Sub-Task 4.4: Spoiler-Free Catch-up Recap (SRS 19) (+2 more)

### Community 26 - "entities/mod.rs"
Cohesion: 0.14
Nodes (13): entity_as_badges, entity_as_bookchunks, entity_as_books, entity_as_booktags, entity_as_chapters, entity_as_readingactivitylogs, entity_as_savedquotes, entity_as_tags (+5 more)

### Community 27 - "AppState"
Cohesion: 0.06
Nodes (84): apiresponse, apperror, asynccommands, auth, axum, books, crate, HttpError (+76 more)

### Community 28 - "pool.rs"
Cohesion: 0.16
Nodes (11): init_db_pool(), init_redis_client(), Client, DatabaseConnection, Result, Self, info, repositories (+3 more)

### Community 29 - "smoke_test.rs"
Cohesion: 0.18
Nodes (5): duration, io, tcpstream, timeout, tracing

### Community 30 - "catalog_test.rs"
Cohesion: 0.33
Nodes (10): Result, String, Uuid, seed_test_catalog(), SeededCatalog, test_book_overview_chapter_and_offline_bundle(), test_catalog_listing_and_filtering(), test_catalog_typo_tolerant_fts_search() (+2 more)

### Community 31 - "2. Step-by-Step Test Procedure"
Cohesion: 0.18
Nodes (11): 2. Step-by-Step Test Procedure, Step 2.10: Logout (Revoke Session), Step 2.1: Register a New User Account, Step 2.2: Retrieve OTP from Mailpit, Step 2.3: Verify OTP and Activate Account, Step 2.4: Inspect Authenticated User Profile, Step 2.5: Update Profile Information, Step 2.6: Change Password (+3 more)

### Community 33 - "Model"
Cohesion: 0.22
Nodes (8): Entity, Model, DateTimeWithTimeZone, Option, Related, RelationDef, String, Uuid

### Community 34 - "manual-test/README.md"
Cohesion: 0.20
Nodes (6): 1. Overview, Manual Testing — Milestone 02: Authentication, Security & Guest Progress, 1. Directory Structure & Milestone Modules, 2. Server & Service URLs Quick Reference, 3. General Testing Workflow, Project Baca — Manual Testing Documentation

### Community 35 - "Manual Testing — Milestone 03: Catalog, Reader Engine & Gamification"
Cohesion: 0.20
Nodes (10): 1. Overview, 2. Quick Data Seeder for Manual Testing, 5. Reading Progress, CFI Anchors & Active Position, 6. Gamification: Heartbeats, Streaks & Badges, Manual Testing — Milestone 03: Catalog, Reader Engine & Gamification, Step 3.10: Send Reading Heartbeat (`POST /api/v1/activity/heartbeat`), Step 3.11: View Master Badges (`GET /api/v1/badges`), Step 3.12: View User Unlocked Badges (`GET /api/v1/me/badges`) (+2 more)

### Community 43 - "2. Documentation Directory"
Cohesion: 0.22
Nodes (9): 1. Architecture & Tech Stack, 2. Documentation Directory, 3. Quickstart, Agentic Intelligence System, AI Agent Governance, Architecture & Operations, Core Specifications (OKF v0.2), Production Runtime (+1 more)

### Community 46 - "Model"
Cohesion: 0.25
Nodes (7): Model, Relation, DateTimeWithTimeZone, Json, Option, String, Uuid

### Community 47 - ".find_book_by_id"
Cohesion: 0.25
Nodes (7): BookCatalogPort, Option, Result, Uuid, test_reliability_mock_repository_fault_injection(), Send, Sync

### Community 48 - "3. Server Health & OpenAPI Documentation"
Cohesion: 0.25
Nodes (8): 1. Overview, 3. Server Health & OpenAPI Documentation, 4. Structured Log Inspection, Manual Testing — Milestone 01: Infrastructure & Configuration, Step 1.5: Start the Axum Server, Step 1.6: Query Server Health Endpoint, Step 1.7: Query Infrastructure Connectivity Endpoint, Step 1.8: Access Swagger UI in Browser

### Community 49 - "Model"
Cohesion: 0.29
Nodes (6): Model, Relation, DateTimeWithTimeZone, String, Uuid, PgVector

### Community 50 - "serde"
Cohesion: 0.29
Nodes (5): Model, Relation, Uuid, Relation, serde

### Community 51 - "Model"
Cohesion: 0.29
Nodes (6): Model, Relation, DateTimeWithTimeZone, Option, String, Uuid

### Community 52 - "Model"
Cohesion: 0.29
Nodes (6): Model, Relation, Date, DateTimeWithTimeZone, Option, Uuid

### Community 53 - "Model"
Cohesion: 0.29
Nodes (6): Model, Relation, DateTimeWithTimeZone, Option, String, Uuid

### Community 54 - "chapters.rs"
Cohesion: 0.33
Nodes (5): Model, Relation, DateTimeWithTimeZone, String, Uuid

### Community 55 - "reading_activity_logs.rs"
Cohesion: 0.33
Nodes (5): Model, Relation, Date, DateTimeWithTimeZone, Uuid

### Community 56 - "tags.rs"
Cohesion: 0.33
Nodes (5): Model, Relation, DateTimeWithTimeZone, String, Uuid

### Community 57 - "Entity"
Cohesion: 0.40
Nodes (4): Entity, Option, Related, RelationDef

### Community 58 - "user_badges.rs"
Cohesion: 0.33
Nodes (5): Model, Relation, DateTimeWithTimeZone, String, Uuid

### Community 59 - "badges.rs"
Cohesion: 0.40
Nodes (4): Model, Relation, DateTimeWithTimeZone, String

### Community 60 - "2. Prerequisites & Service Status Verification"
Cohesion: 0.40
Nodes (5): 2. Prerequisites & Service Status Verification, Step 1.1: Verify Docker Containers, Step 1.2: Check PostgreSQL Schema & Extensions, Step 1.3: Check Redis Connectivity, Step 1.4: Check Mailpit & MinIO HTTP Endpoints

### Community 61 - "3. Catalog Discovery & Typo-Tolerant Search"
Cohesion: 0.40
Nodes (5): 3. Catalog Discovery & Typo-Tolerant Search, Step 3.1: Browse Catalog (`GET /api/v1/books`), Step 3.2: Filter by Theme & Language, Step 3.3: Typo-Tolerant FTS Search (`GET /api/v1/books/search`), Step 3.4: Query Validation

### Community 62 - "middleware/mod.rs"
Cohesion: 0.50
Nodes (3): authuser, rate_limit_middleware, require_admin

### Community 63 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 64 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 65 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 66 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 67 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 68 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 69 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 70 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 71 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 72 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 73 - "Entity"
Cohesion: 0.50
Nodes (3): Entity, Related, RelationDef

### Community 74 - "4. Book Overview, Reader Content & Offline Bundle"
Cohesion: 0.50
Nodes (4): 4. Book Overview, Reader Content & Offline Bundle, Step 3.5: Book Overview & Chapter Summary (`GET /api/v1/books/{id}`), Step 3.6: Reader Chapter Delivery (`GET /api/v1/books/{id}/chapters/{chapter_number}`), Step 3.7: Offline Bundle Synchronization (`GET /api/v1/books/{id}/offline-bundle`)

### Community 90 - "rate_limit_middleware"
Cohesion: 0.29
Nodes (7): rate_limit_middleware(), Arc, Body, Next, Request, Response, State

### Community 91 - "Q: nah pertanyaanmu terkait graf seriusan ga paham, apalagi across architetureal boundaries gitu. tolong deh telusuri jalur data dan dependencynya."
Cohesion: 0.40
Nodes (4): Answer, Outcome, Q: nah pertanyaanmu terkait graf seriusan ga paham, apalagi across architetureal boundaries gitu. tolong deh telusuri jalur data dan dependencynya., Source Nodes

## Knowledge Gaps
- **144 isolated node(s):** `Relation`, `ActiveModel`, `Relation`, `ActiveModel`, `Relation` (+139 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 350 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **30 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppError` connect `AppError` to `shared/src/lib.rs`, `.find_book_by_id`, `AppConfig`, `progress_repository.rs`, `AppState`, `pool.rs`, `smoke_test.rs`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Why does `AppState` connect `AppState` to `Test Harness Utilities`, `AppConfig`, `rate_limit_middleware`, `server/src/lib.rs`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `AppConfig` to `AppState`, `server/src/lib.rs`, `pool.rs`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **What connects `Relation`, `ActiveModel`, `Relation` to the rest of the system?**
  _144 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `reliability_test.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12554112554112554 - nodes in this community are weakly interconnected._
- **Should `Database Schema and Indexes` be split into smaller, more focused modules?**
  _Cohesion score 0.11229946524064172 - nodes in this community are weakly interconnected._
- **Should `shared/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10377358490566038 - nodes in this community are weakly interconnected._