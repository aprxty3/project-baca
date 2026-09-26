# Project Baca — Operational Status & Backlog Log

Current status, technical milestones, and operational backlog for **Project Baca**.

## 1. Current State

* **Target:** E-Reader Apps (Web PWA Leptos WASM, Mobile via Tauri v2, RAG/Semantic Subsystem).
* **Local Infrastructure:**
  * Rust Toolchain: Stable (1.80+), `wasm32-unknown-unknown` target active.
  * WASM Bundler: Trunk.
  * Docker Compose: PostgreSQL 17 + pgvector (5433), Redis 7 (6380), MinIO S3 (9005/9006), Mailpit (1025/8025).
* **Core Artifacts Completed:**
  * [ARCHITECTURE.md](ARCHITECTURE.md) (Polyglot Monorepo Blueprint, Ingestion Pipeline, Scoped Quote Search).
  * [README.md](README.md) (Tech stack summary and quickstart).
  * Specifications in `knowledge/` (`prd.md`, `erd.md`, `ux-flow.md`, `frd.md`, `srs.md`, `index.md`, `log.md`).
  * Visual assets cataloged in [assets/README.md](assets/README.md).

## 2. Implementation Backlog (Phase 1 to MVP)

1. **[COMPLETED] Phase 0 — Monorepo Foundation & Database Migrations:**
   - 5-crate Cargo workspace (`domain`, `shared`, `infra`, `server`, `web`).
   - 6 paired SQL migrations applied to PostgreSQL 17 (14 tables, 37 indexes).
   - OpenAPI Swagger UI (`utoipa`) integrated at `/swagger-ui`.
   - Structured JSON logging with `x-request-id` propagation.
   - Comprehensive 4-tier test suite (smoke, integration, performance, reliability).
2. **[COMPLETED] Milestone 01 — Infrastructure, Modular AppConfig & SeaORM Entities:**
   - Spec: [knowledge/tasks/01_infrastructure_and_config.md](knowledge/tasks/01_infrastructure_and_config.md)
   - Scope: `dotenvy` integration, strongly-typed modular configurations (`ServerConfig`, `DatabaseConfig`, `RedisConfig`, `AuthConfig`, `StorageConfig`, `EmailConfig`, `AiConfig`), dynamic SeaORM and Redis connection pools, 13 SeaORM entity definitions with `postgres-vector` (`PgVector`) support, Arc-wrapped `AppState`.
3. **[COMPLETED] Milestone 02 — Authentication, JWT/Argon2id, OTP & Guest Reconciliation:**
   - Spec: [knowledge/tasks/02_authentication_and_user.md](knowledge/tasks/02_authentication_and_user.md)
   - Scope: End-to-end auth lifecycle (`/api/v1/auth/signup`, `verify-otp`, `login`, `refresh`, `logout`), profile management (`/api/v1/me`), Argon2id password hashing, Redis OTP (SHA-256 with 3-attempt limit), JWT access token (24h) & 14-day refresh token rotation, Mailpit SMTP email dispatch, guest progress upsert reconciliation (`/api/v1/progress/merge`), rate limiting & auth middleware. Verified with 100% test pass rate.
4. **[COMPLETED] OWASP ASVS Hardening & Performance Optimization (Auth, Query Repository & Vulnerability Suite):**
   - Scope: Error sanitization suppressing internal database/system leakage (CWE-209 mitigation), structured validation error details, global OWASP security headers, IP spoofing defense with Cloudflare `CF-Connecting-IP`, RFC rate limit headers (`X-RateLimit-*`, `Retry-After`), 60s email OTP anti-spam cooldown, 15-min brute-force lockout on 5 failed logins, JWT `jti` access token blacklisting on logout, user-level token invalidation timestamps, user refresh token tracking set with `POST /api/v1/auth/revoke-all`, cascading session cleanup on account deletion (`DELETE /api/v1/me`).
   - Query & Latency Optimization: Extracted database access into dedicated `user_repository` with atomic activation; eliminated TCP handshake churn by caching and cloning multiplexed Redis connection in `AppState` (`state.get_redis_conn().await`); calibrated Argon2id to OWASP guidelines (19MB, 2 iterations, 1 lane) and offloaded to Tokio blocking thread pool; mitigated timing attacks via constant-time dummy verification on non-existent users; configured `[profile.dev.package.argon2]` and `blake2` with `opt-level = 3` for snappy dev performance.
   - Test Suite: Verified with 10 OWASP security tests, 4 performance benchmarks (Login p95 = 52ms, OTP verify p95 = 26ms), 5 smoke tests, 7 integration tests, and 3 reliability tests (41 total workspace tests passing).
5. **[COMPLETED] Milestone 03 — Catalog Backend, Trigram FTS, Reader API & Streak Gamification:**
   - Spec: [knowledge/tasks/03_catalog_and_reader_backend.md](knowledge/tasks/03_catalog_and_reader_backend.md)
   - Scope: Cursor-based catalog (`/api/v1/books`), typo-tolerant FTS search (<3ms via `pg_trgm` and `word_similarity`), book overview, chapter content delivery, offline bundle synchronization, CFI reading progress sync (`/api/v1/progress/{book_id}`), active position retrieval (`/api/v1/progress/active`), reading heartbeat & daily streak engine (`/api/v1/activity/heartbeat`), and badges gamification (`/api/v1/badges`, `/api/v1/me/badges`). Verified with 100% test pass rate.
5. **[PENDING] Milestone 04 — Semantic AI Subsystem, 768-Dim Embeddings & Atomic Cards:**
   - Spec: [knowledge/tasks/04_semantic_ai_and_insights.md](knowledge/tasks/04_semantic_ai_and_insights.md)
   - Scope: Dual-mode embeddings (Gemini/FastEmbed), Scoped Quote Finder HNSW (<10ms), chapter atomic cards.
6. **[PENDING] Milestone 05 — Asynchronous EPUB Ingestion Pipeline & Admin Management:**
   - Spec: [knowledge/tasks/05_ingestion_pipeline_and_admin.md](knowledge/tasks/05_ingestion_pipeline_and_admin.md)
   - Scope: Admin EPUB upload, Redis Streams worker, HTML sanitization, scene chunking, processing monitor.
7. **[PENDING] Milestone 06 — Frontend Leptos WASM Web Reader & Offline Mode:**
   - Spec: [knowledge/tasks/06_frontend_leptos_web_reader.md](knowledge/tasks/06_frontend_leptos_web_reader.md)
   - Scope: Leptos WASM SPA, Vintage Literary theme, multi-column reader, CFI anchors, IndexedDB (`rexie`), i18n switcher.
8. **[PENDING] Milestone 07 — Quality Assurance, SLA Benchmarks & MVP Launch:**
   - Spec: [knowledge/tasks/07_quality_assurance_and_launch.md](knowledge/tasks/07_quality_assurance_and_launch.md)
   - Scope: End-to-end testing, SLA verification (API <50ms, FTS <5ms, quote <10ms), Zero Panic and Zero Emoji audits.

## 3. Architecture Decision Records (ADR Summary)

Full rationales documented in [MEMORY.md](MEMORY.md):

| ID | Domain | Technical Decision | Authoritative Source |
|---|---|---|---|
| ADR-01 | Architecture | Polyglot Monorepo (Rust Axum + Leptos WASM + Python Worker + Dual-Mode Embeddings) | [MEMORY.md](MEMORY.md) |
| ADR-02 | Database & Search | "Postgres for Everything" (FTS + Trigrams + pgvector HNSW); rejected Elasticsearch | [MEMORY.md](MEMORY.md) |
| ADR-03 | Message Broker | Redis Streams for ingestion and email delivery; rejected RedPanda/RabbitMQ | [MEMORY.md](MEMORY.md) |
| ADR-04 | Data Layer | SeaORM async ORM + paired SQL migrations via Makefile | [MEMORY.md](MEMORY.md) |
| ADR-05 | Design System | Vintage Literary (1900–1950) theme + atomic insight cards | [MEMORY.md](MEMORY.md) |
| ADR-06 | Engineering Rules | Strict 6 Pillars (ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, YAGNI) & Zero Emoji Policy | [MEMORY.md](MEMORY.md) |
| ADR-07 | Knowledge System | OKF v0.2 Knowledge Vault and Graphify codebase AST extraction | [MEMORY.md](MEMORY.md) |
| ADR-08 | Agent Governance | Publication of AGENTS.md, CLAUDE.md, GEMINI.md, GUIDE.md, DISTRIBUTED.md | [MEMORY.md](MEMORY.md) |
| ADR-09 | Vector Embeddings | Eliminated Triton Server; standardized on 768-dim Google GenAI API & FastEmbed CPU | [MEMORY.md](MEMORY.md) |
| ADR-10 | Knowledge Graph | Maintained synchronized codebase AST and documentation graph | [MEMORY.md](MEMORY.md) |
| ADR-11 | Monorepo Base | 5-crate Rust workspace + Makefile and Trunk WASM bundler | [MEMORY.md](MEMORY.md) |
| ADR-12 | Database Migrations | 6 PostgreSQL 17 migrations, 16 ERD indexes, Makefile automation | [MEMORY.md](MEMORY.md) |
| ADR-13 | API Tooling & Tests | OpenAPI/Swagger UI (`utoipa`), auto-reload, structured JSON logs, 4-tier test suite | [MEMORY.md](MEMORY.md) |
| ADR-14 | Infrastructure Layer | Strongly-typed modular `AppConfig`, dynamic connection pools, and 13 SeaORM entities with `postgres-vector` | [MEMORY.md](MEMORY.md) |
