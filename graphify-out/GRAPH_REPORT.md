# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- 61 files · ~186,190 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 282 nodes · 323 edges · 63 communities (23 shown, 40 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 10 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ed85f34e`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- shared/src/lib.rs
- AppConfig
- README.md
- domain/src/lib.rs
- 2. Work Breakdown
- 2. Work Breakdown
- 2. Work Breakdown
- 2. Work Breakdown
- 2. Work Breakdown
- knowledge/index.md — Master Knowledge Catalog
- 2. Work Breakdown
- migrate.sh
- shared
- knowledge/frd.md — Functional Requirements Document
- App
- Security Architecture
- User Journey Flowchart
- Graphify Knowledge Graph Protocol
- ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain
- Admin Sorting Illustration
- DISTRIBUTED.md — Arsitektur Pemrosesan Terdistribusi
- GET /api/books/{id}/offline-bundle
- Performance Metrics & SLA
- knowledge/tasks/README.md — Master Task Roadmap
- Reader Interface Design
- Library Catalog Design
- Ingestion Monitoring Interface
- Asset Management
- Project Changelog
- Web Entry Point
- Developer Operations Guide
- SMTP Development Service
- Object Storage Service
- Vector Database Infrastructure
- Cache Infrastructure
- Activity Heartbeat Endpoint
- Ingestion Job Monitoring
- Book Upload Endpoint
- API Error Standards
- Atomic Insights Endpoint
- User Login Endpoint
- User Logout Endpoint
- User Signup Endpoint
- Chapter Content Endpoint
- Book Detail Endpoint
- Book Catalog Endpoint
- Contextual Recap Endpoint
- Software Requirements Specification
- User Account Deletion
- User Profile Endpoint
- Reading Progress Update
- Core UX Philosophy
- Legal and Licensing
- PostgreSQL 17 + pgvector
- README.md — Project Baca Overview
- Redis 7
- server/src/lib.rs
- TestHarness
- reliability_test.rs

## God Nodes (most connected - your core abstractions)
1. `AppConfig` - 8 edges
2. `main()` - 8 edges
3. `2. Work Breakdown` - 8 edges
4. `Book` - 7 edges
5. `init_db_pool()` - 7 edges
6. `init_redis_client()` - 7 edges
7. `AppState` - 7 edges
8. `create_app()` - 7 edges
9. `TestHarness` - 7 edges
10. `2. Work Breakdown` - 7 edges

## Surprising Connections (you probably didn't know these)
- `AI Quote Finder Illustration` --conceptually_related_to--> `knowledge/prd.md — Product Requirements Document`  [EXTRACTED]
  assets/illustrations/retro-rocket-discovery.png → knowledge/prd.md
- `GBrain Design Reference` --conceptually_related_to--> `ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain`  [EXTRACTED]
  assets/references/gbrain-design-reference.png → ARCHITECTURE.md
- `Admin Sorting Illustration` --conceptually_related_to--> `knowledge/ux-flow.md — UX Flow & Navigation`  [EXTRACTED]
  assets/illustrations/admin-sorting-pigeonholes.png → knowledge/ux-flow.md
- `test_reliability_disconnected_database_fallback()` --calls--> `create_app()`  [INFERRED]
  crates/server/tests/reliability_test.rs → crates/server/src/lib.rs
- `DISTRIBUTED.md — Arsitektur Pemrosesan Terdistribusi` --references--> `Docker Infrastructure Configuration`  [INFERRED]
  DISTRIBUTED.md → docker-compose.yml

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Authentication & Account Management Flow** — knowledge_srs_auth_signup, knowledge_srs_auth_verify_otp, knowledge_srs_auth_login, knowledge_srs_auth_refresh, knowledge_srs_auth_logout [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **MVP Development Critical Path** — knowledge_tasks_01_infrastructure_and_config, knowledge_tasks_02_authentication_and_user, knowledge_tasks_03_catalog_and_reader_backend, knowledge_tasks_04_semantic_ai_and_insights, knowledge_tasks_05_ingestion_pipeline_and_admin, knowledge_tasks_06_frontend_leptos_web_reader, knowledge_tasks_07_quality_assurance_and_launch [EXTRACTED 1.00]
- **Semantic Intelligence Cluster** — knowledge_srs_quotes_search, knowledge_srs_atomic_cards, knowledge_srs_chapter_recap [EXTRACTED 1.00]
- **Core Reading Experience** — knowledge_srs_books_chapter, knowledge_srs_progress_sync, knowledge_srs_chapter_recap, knowledge_ux_flow_philosophy [INFERRED 0.90]

## Communities (63 total, 40 thin omitted)

### Community 0 - "shared/src/lib.rs"
Cohesion: 0.16
Nodes (23): ApiResponse, ApiResponse<T>, BookDetailDto, BookSummaryDto, ChapterSummaryDto, ErrorPayload, LoginRequest, QuoteSearchRequest (+15 more)

### Community 1 - "AppConfig"
Cohesion: 0.16
Nodes (15): Box, AppConfig, init_db_pool(), init_redis_client(), Client, DatabaseConnection, Result, Self (+7 more)

### Community 2 - "README.md"
Cohesion: 0.11
Nodes (16): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 1.1: `dotenvy` Integration, Sub-Task 1.2: Modular `AppConfig` Sub-Configurations, Sub-Task 1.3: Dynamic Connection Pools, Sub-Task 1.4: SeaORM Entities for 13 Tables (+8 more)

### Community 3 - "domain/src/lib.rs"
Cohesion: 0.23
Nodes (17): Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak, DateTime (+9 more)

### Community 4 - "2. Work Breakdown"
Cohesion: 0.15
Nodes (12): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 6.1: Vintage Literary Design System (1900–1950), Sub-Task 6.2: API Client & Local-First IndexedDB (`rexie`), Sub-Task 6.3: Home Page (Catalog & Search), Sub-Task 6.4: Book Overview Page (+4 more)

### Community 5 - "2. Work Breakdown"
Cohesion: 0.17
Nodes (11): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 5.1: S3/MinIO Storage Service (`crates/infra`), Sub-Task 5.2: Admin EPUB Upload Endpoint (SRS 21), Sub-Task 5.3: Ingestion Worker & EPUB Parsing, Sub-Task 5.4: HTML Sanitization & Scene Chunking (+3 more)

### Community 6 - "2. Work Breakdown"
Cohesion: 0.17
Nodes (11): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 7.1: Smoke Testing Suite, Sub-Task 7.2: Unit Test Suite, Sub-Task 7.3: Integration Test Suite, Sub-Task 7.4: Performance SLA Benchmarks (+3 more)

### Community 7 - "2. Work Breakdown"
Cohesion: 0.18
Nodes (10): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 2.1: Password Hashing (Argon2id) & OTP, Sub-Task 2.2: JWT Tokens & Refresh Rotation, Sub-Task 2.3: REST API Endpoints (SRS 1–7), Sub-Task 2.4: Guest Reconciliation (`POST /api/progress/merge`) (+2 more)

### Community 8 - "2. Work Breakdown"
Cohesion: 0.18
Nodes (10): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 4.1: Dual-Mode Embedding Provider (`crates/infra`), Sub-Task 4.2: Scoped Semantic Quote Finder (`POST /api/books/{id}/quotes/search`), Sub-Task 4.3: Chapter Atomic Insight Cards (SRS 18), Sub-Task 4.4: Spoiler-Free Catch-up Recap (SRS 19) (+2 more)

### Community 9 - "knowledge/index.md — Master Knowledge Catalog"
Cohesion: 0.29
Nodes (8): AGENTS.md — Panduan & Tata Kelola Agen AI, OKF & Persistent Memory Guidelines, CLAUDE.md — Claude Code Guidelines, GEMINI.md — Panduan Gemini & Google Antigravity, knowledge/index.md — Master Knowledge Catalog, knowledge/log.md — Audit Trail, MEMORY.md — Log Keputusan Arsitektur & Memori Sistem, Open Knowledge Format v0.2

### Community 10 - "2. Work Breakdown"
Cohesion: 0.20
Nodes (9): 1. Summary, 2. Work Breakdown, 3. Success Criteria, Cross-Domain Matrix, Sub-Task 3.1: Catalog Repository & FTS Lexical Search, Sub-Task 3.2: Catalog & Chapter Endpoints (SRS 8–12), Sub-Task 3.3: Reading Progress & CFI Synchronization (SRS 13–14), Sub-Task 3.4: Reading Heartbeat, Streaks & Badges (SRS 15–16) (+1 more)

### Community 11 - "migrate.sh"
Cohesion: 0.76
Nodes (6): init_table(), migrate_down(), migrate_status(), migrate_up(), psql_cmd(), migrate.sh script

### Community 12 - "shared"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 13 - "knowledge/frd.md — Functional Requirements Document"
Cohesion: 0.50
Nodes (4): AI Quote Finder Illustration, knowledge/erd.md — Entity Relationship Diagram, knowledge/frd.md — Functional Requirements Document, knowledge/prd.md — Product Requirements Document

### Community 15 - "Security Architecture"
Cohesion: 0.67
Nodes (3): POST /api/auth/refresh, POST /api/auth/verify-otp, Security Architecture

### Community 16 - "User Journey Flowchart"
Cohesion: 0.67
Nodes (3): GET /api/books/search, POST /api/progress/merge, User Journey Flowchart

### Community 57 - "server/src/lib.rs"
Cohesion: 0.16
Nodes (16): api_health_check(), ApiDoc, AppState, create_app(), health_check(), request_id_middleware(), Arc, Body (+8 more)

### Community 58 - "TestHarness"
Cohesion: 0.36
Nodes (7): Arc, Body, Request, Response, Router, Value, TestHarness

### Community 59 - "reliability_test.rs"
Cohesion: 0.29
Nodes (4): BookCatalogPort, test_reliability_disconnected_database_fallback(), Send, Sync

## Knowledge Gaps
- **110 isolated node(s):** `ApiDoc`, `web`, `Cross-Domain Matrix`, `Sub-Task 1.1: `dotenvy` Integration`, `Sub-Task 1.2: Modular `AppConfig` Sub-Configurations` (+105 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **40 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppError` connect `AppConfig` to `shared/src/lib.rs`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **Why does `AppConfig` connect `AppConfig` to `server/src/lib.rs`, `reliability_test.rs`?**
  _High betweenness centrality (0.026) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `main()` (e.g. with `init_db_pool()` and `init_redis_client()`) actually correct?**
  _`main()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ApiDoc`, `web`, `Cross-Domain Matrix` to the rest of the system?**
  _110 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `README.md` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._