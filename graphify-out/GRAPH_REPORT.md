# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 329 nodes · 443 edges · 47 communities (17 shown, 30 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 8 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `03fd87ca`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Integration and Performance Testing
- Database Schema and Indexes
- Shared Data Transfer Objects
- API Server Configuration
- Auth and Security Tasks
- Domain Entities and Logic
- Infrastructure and Client Initialization
- Frontend Web Reader Tasks
- Test Harness Utilities
- Infrastructure and SeaORM Setup
- EPUB Ingestion Pipeline Tasks
- Quality Assurance and Benchmarking
- Project Documentation and Diagrams
- Semantic AI Subsystem Tasks
- Catalog and Progress Tasks
- Database Migration Scripts
- Project Workspace Modules
- Architecture and Design References
- Web Frontend Components
- Knowledge Graph Protocols
- Web Entry Assets
- Reader UI Illustrations
- Catalog UI Illustrations
- Monitoring UI Illustrations
- API Client
- Database Connection
- Date and Time Types
- String Type
- UTC Timezone
- Knowledge Format Specification
- Database Technology Stack

## God Nodes (most connected - your core abstractions)
1. `books` - 12 edges
2. `users` - 8 edges
3. `main()` - 8 edges
4. `2. Work Breakdown` - 8 edges
5. `AppState` - 7 edges
6. `AppError` - 7 edges
7. `TestHarness` - 7 edges
8. `chapters` - 7 edges
9. `2. Work Breakdown` - 7 edges
10. `2. Work Breakdown` - 7 edges

## Surprising Connections (you probably didn't know these)
- `Web Entry Point` --semantically_similar_to--> `Asset Catalog`  [INFERRED] [semantically similar]
  crates/web/index.html → assets/README.md
- `main()` --calls--> `create_app()`  [INFERRED]
  crates/server/src/main.rs → crates/server/src/lib.rs
- `test_reliability_disconnected_database_fallback()` --calls--> `create_app()`  [INFERRED]
  crates/server/tests/reliability_test.rs → crates/server/src/lib.rs
- `TestHarness` --references--> `AppState`  [EXTRACTED]
  crates/server/tests/common/mod.rs → crates/server/src/lib.rs
- `user_reading_streaks` --references--> `users`  [EXTRACTED]
  migrations/20260925000006_create_progress_and_gamification.up.sql → migrations/20260925000002_create_users_and_roles.up.sql

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Infrastructure Services** — redis_7 [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]
- **MVP Implementation Sequence** — knowledge_tasks_01_infrastructure_and_config, knowledge_tasks_02_authentication_and_user, knowledge_tasks_03_catalog_and_reader_backend, knowledge_tasks_04_semantic_ai_and_insights, knowledge_tasks_05_ingestion_pipeline_and_admin, knowledge_tasks_06_frontend_leptos_web_reader, knowledge_tasks_07_quality_assurance_and_launch [EXTRACTED 1.00]
- **OKF Knowledge Vault** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]

## Communities (47 total, 30 thin omitted)

### Community 0 - "Integration and Performance Testing"
Cohesion: 0.06
Nodes (23): arc, assert_eq, automock, body, BookCatalogPort, Option, Result, Uuid (+15 more)

### Community 1 - "Database Schema and Indexes"
Cohesion: 0.11
Nodes (29): idx_users_admin_role, idx_users_email, users, book_tags, books, idx_book_tags_reverse, idx_books_catalog_filter, idx_books_published_author_trgm (+21 more)

### Community 2 - "Shared Data Transfer Objects"
Cohesion: 0.14
Nodes (26): chrono, ApiResponse, ApiResponse<T>, BookDetailDto, BookSummaryDto, ChapterSummaryDto, ErrorPayload, LoginRequest (+18 more)

### Community 3 - "API Server Configuration"
Cohesion: 0.10
Nodes (25): AppConfig, axum, Client, cors, api_health_check(), ApiDoc, AppState, create_app() (+17 more)

### Community 4 - "Auth and Security Tasks"
Cohesion: 0.11
Nodes (21): OKF & Persistent Memory Guidelines, Task 02: Auth & Security, 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 2.1: Password Hashing (Argon2id) & OTP, Sub-Task 2.2: JWT Tokens & Refresh Rotation (+13 more)

### Community 5 - "Domain Entities and Logic"
Cohesion: 0.23
Nodes (17): Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak, DateTime (+9 more)

### Community 6 - "Infrastructure and Client Initialization"
Cohesion: 0.17
Nodes (15): Box, AppConfig, init_db_pool(), init_redis_client(), Client, DatabaseConnection, Result, Self (+7 more)

### Community 7 - "Frontend Web Reader Tasks"
Cohesion: 0.17
Nodes (12): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 6.1: Vintage Literary Design System (1900–1950), Sub-Task 6.2: API Client & Local-First IndexedDB (`rexie`), Sub-Task 6.3: Home Page (Catalog & Search), Sub-Task 6.4: Book Overview Page (+4 more)

### Community 8 - "Test Harness Utilities"
Cohesion: 0.31
Nodes (8): Arc, Body, Request, Response, Router, Value, TestHarness, serviceext

### Community 9 - "Infrastructure and SeaORM Setup"
Cohesion: 0.18
Nodes (11): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 1.1: `dotenvy` Integration, Sub-Task 1.2: Modular `AppConfig` Sub-Configurations, Sub-Task 1.3: Dynamic Connection Pools, Sub-Task 1.4: SeaORM Entities for 13 Tables (+3 more)

### Community 10 - "EPUB Ingestion Pipeline Tasks"
Cohesion: 0.18
Nodes (11): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 5.1: S3/MinIO Storage Service (`crates/infra`), Sub-Task 5.2: Admin EPUB Upload Endpoint (SRS 21), Sub-Task 5.3: Ingestion Worker & EPUB Parsing, Sub-Task 5.4: HTML Sanitization & Scene Chunking (+3 more)

### Community 11 - "Quality Assurance and Benchmarking"
Cohesion: 0.18
Nodes (11): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 7.1: Smoke Testing Suite, Sub-Task 7.2: Unit Test Suite, Sub-Task 7.3: Integration Test Suite, Sub-Task 7.4: Performance SLA Benchmarks (+3 more)

### Community 13 - "Semantic AI Subsystem Tasks"
Cohesion: 0.20
Nodes (10): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 4.1: Dual-Mode Embedding Provider (`crates/infra`), Sub-Task 4.2: Scoped Semantic Quote Finder (`POST /api/books/{id}/quotes/search`), Sub-Task 4.3: Chapter Atomic Insight Cards (SRS 18), Sub-Task 4.4: Spoiler-Free Catch-up Recap (SRS 19) (+2 more)

### Community 14 - "Catalog and Progress Tasks"
Cohesion: 0.22
Nodes (9): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 3.1: Catalog Repository & FTS Lexical Search, Sub-Task 3.2: Catalog & Chapter Endpoints (SRS 8–12), Sub-Task 3.3: Reading Progress & CFI Synchronization (SRS 13–14), Sub-Task 3.4: Reading Heartbeat, Streaks & Badges (SRS 15–16) (+1 more)

### Community 15 - "Database Migration Scripts"
Cohesion: 0.76
Nodes (6): init_table(), migrate_down(), migrate_status(), migrate_up(), psql_cmd(), migrate.sh script

### Community 16 - "Project Workspace Modules"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 18 - "Web Frontend Components"
Cohesion: 0.50
Nodes (3): App(), IntoView, prelude

## Knowledge Gaps
- **70 isolated node(s):** `ApiDoc`, `3. Success Criteria`, `Cross-Domain Matrix`, `Sub-Task 5.1: S3/MinIO Storage Service (`crates/infra`)`, `Sub-Task 5.2: Admin EPUB Upload Endpoint (SRS 21)` (+65 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 156 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **30 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppError` connect `Infrastructure and Client Initialization` to `Integration and Performance Testing`, `Shared Data Transfer Objects`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `main()` connect `Infrastructure and Client Initialization` to `Integration and Performance Testing`, `API Server Configuration`?**
  _High betweenness centrality (0.021) - this node is a cross-community bridge._
- **Why does `create_app()` connect `API Server Configuration` to `Infrastructure and Client Initialization`?**
  _High betweenness centrality (0.020) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `main()` (e.g. with `init_db_pool()` and `init_redis_client()`) actually correct?**
  _`main()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ApiDoc`, `3. Success Criteria`, `Cross-Domain Matrix` to the rest of the system?**
  _70 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Integration and Performance Testing` be split into smaller, more focused modules?**
  _Cohesion score 0.0627177700348432 - nodes in this community are weakly interconnected._
- **Should `Database Schema and Indexes` be split into smaller, more focused modules?**
  _Cohesion score 0.11229946524064172 - nodes in this community are weakly interconnected._