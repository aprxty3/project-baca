---
type: Memory
title: "Project Baca — Architecture Decision Records & System Memory"
description: "Chronological records of technical and architectural decisions and their underlying rationale."
tags: [memory, decisions, architecture, history, context, okf]
---

# MEMORY.md — Architecture Decision Records & System Memory

Technical rationales behind engineering decisions for **Project Baca**.

## Thematic Decision Index

* **Core Architecture:** Polyglot monorepo (Rust Axum + Leptos WASM + Python Worker + Dual-Mode Embeddings).
* **Vector Engine:** Eliminated Triton Server; adopted Google GenAI API (cloud) & FastEmbed CPU (ARM64/x86); standardized on 768-dimensional embeddings.
* **Database & Search:** "Postgres for Everything" (FTS + `pg_trgm` + `pgvector` HNSW); rejected Elasticsearch.
* **Message Broker:** Redis Streams for ingestion and email; rejected RedPanda/RabbitMQ for MVP.
* **Data Layer & Migrations:** SeaORM with paired SQL migrations (`.up.sql` and `.down.sql`).
* **Object Storage & Email:** S3-compatible (MinIO / Cloudflare R2); Mailpit for local SMTP.
* **User Experience:** Mid-century Vintage Literary (1900–1950) aesthetic; reactive bilingual UI switching (`[ID|EN]`).
* **Engineering Invariants:** 6 Pillars (ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, YAGNI) and Zero Emoji Policy.
* **Knowledge Governance:** Open Knowledge Format (OKF v0.2), Graphify AST extraction, and Obsidian Vault.

## Chronological Decision Records

### 2026-09-26 — Comprehensive Testing Architecture (Database Integrity, Load & Stress, API Boundary)
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Preventing technical debt and regression while the codebase is compact requires proactive automated testing covering: (1) database constraints, cascades, rollbacks, and query plans; (2) sustained high concurrency load and rate-limit shedding; (3) HTTP edge-cases, 404 envelopes, 405 methods, and malformed body handling.
* **Decision:**
  1. Dedicated Database Suite (`database_test.rs`): Explicitly asserted schema constraints (`users.email` unique, `tags.slug` unique, `users.role` check), PostgreSQL foreign key cascades (`users` -> `progress`/`badges`, `books` -> `chapters`/`tags`), transaction rollback cleanliness, and index query planner utilization (`idx_users_email`).
  2. Load & Stress Suite (`load_stress_test.rs`): Tested sustained concurrent load (200 tasks > 220 req/sec in 900ms, 100 authenticated reads > 120 req/sec), validated graceful rate-limit shedding (40 requests -> 20 processed, 20 shed with 429 and `Retry-After`), and burst recovery across distinct IPs.
  3. API Boundary Suite (`api_boundary_test.rs`): Added global `not_found_handler` fallback in Axum returning standard `ApiResponse` JSON envelopes on unknown routes; verified 405 Method Not Allowed, malformed JSON body handling without server panic, empty body rejections, pagination extremes (`limit=0`, `limit=1000`, invalid cursor string), and authorization scheme validation.
  4. Automation: Exposed dedicated commands in `Makefile` (`test-database`, `test-load-stress`, `test-api-boundary`, `test-security`). Full workspace test suite passes with 100% success rate (65 tests across all 5 crates).

### 2026-09-26 — Auth Latency Optimization, Query Repository & Timing Attack Mitigation
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Initial profiling of authentication endpoints revealed latency bottlenecks: (1) each HTTP request opened separate TCP connections to Redis in rate-limit middleware and handlers; (2) CPU-bound Argon2id ran synchronously on Tokio worker threads with non-optimal parameters (64MB, 3 iterations, 4 lanes), starving async executors; (3) non-existent account logins were susceptible to timing enumeration attacks; (4) user queries were ad-hoc in route handlers rather than encapsulated.
* **Decision:**
  1. Caching & Multiplexing: Pre-initialized a multiplexed connection in `AppState` (`state.get_redis_conn().await`), allowing sub-microsecond in-memory handle cloning without socket re-creation.
  2. OWASP Parameter Calibration: Aligned Argon2id with OWASP recommendations (19MB RAM, 2 iterations, 1 parallelism lane) and offloaded execution to Tokio blocking threads (`tokio::task::spawn_blocking`).
  3. Constant-Time Dummy Verification: Mitigated user enumeration by verifying against a static dummy hash (`DUMMY_ARGON2_HASH`) when the user email does not exist.
  4. Repository Encapsulation: Extracted all user database interactions into `crates/infra/src/repositories/user_repository.rs` for clean separation and single-query atomic activations.
  5. Dev Profile Crypto Optimization: Configured `[profile.dev.package.argon2]` and `blake2` with `opt-level = 3` in `Cargo.toml`.
  6. Results: Login p95 latency reduced from >600ms to 52ms; OTP verification p95 reduced to 26ms.

### 2026-09-26 — OWASP ASVS Architecture Hardening (Auth, Rate Limiting & Token Revocation)
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Auditing Authentication and User Management against OWASP ASVS and API Security standards revealed potential attack vectors: information leakage in DB errors (CWE-209), IP spoofing in rate limits, credential stuffing via unthrottled login attempts, OTP spamming on signup, and unrevokable stateless JWT access tokens upon logout or account deletion.
* **Decision:**
  1. Sanitized all internal database and system errors returned to clients while logging full traces with `tracing::error!` and request IDs. Structured validation errors into field maps.
  2. Injected mandatory OWASP security headers (`nosniff`, `DENY`, `strict-origin-when-cross-origin`, `x-xss-protection: 0`) globally.
  3. Integrated RFC rate limit headers (`X-RateLimit-*`, `Retry-After`) with strict IP format validation (Cloudflare `CF-Connecting-IP` priority).
  4. Added account-specific anti-abuse controls: 60-second OTP cooldown per email (`otp:cooldown:{email}`) and 15-minute lockout after 5 failed login attempts (`auth:login:lockout:{email}`).
  5. Implemented hybrid token revocation: embedded unique `jti: Uuid` in JWT access tokens, Redis blacklisting for logged-out access tokens (`blacklist:jti:{jti}`), user-level timestamp invalidation (`user_revoked_before:{user_id}`) for password changes and account deletions, and tracked active refresh token sets (`user_refresh_tokens:{user_id}`) with `POST /api/v1/auth/revoke-all`.

### 2026-09-26 — Graph Mapping & Lifecycle Monorepo Pipeline
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Navigating development milestones requires clear dependencies across Rust crates, database schemas, and specifications.
* **Decision:**
  1. Implemented causal prerequisites and critical paths in task specifications.
  2. Mapped cross-domain relations connecting code crates (`crates/*`), SQL migrations (`migrations/*`), and specs (`knowledge/*`).
  3. Deployed `obsidian-vault/pipeline.canvas` for interactive sprint visualization.
  4. Registered `baca-knowledge` source within GBrain for persistent memory.

### 2026-09-25 — Eliminated Triton Server & Adopted 768-Dim Dual-Mode Embeddings
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Local development lacks dedicated GPUs, and target production uses ARM CPUs. Running Triton containers consumed 10–15 GB disk and 1–2 GB idle RAM without GPU acceleration benefits.
* **Decision:**
  1. Eliminated Triton Server following KISS and YAGNI.
  2. Adopted Dual-Mode Embedding Provider:
     - Cloud Mode: Google GenAI API (`text-embedding-004`) for zero-footprint vector generation.
     - Local Mode: CPU-optimized ONNX FastEmbed (<150 MB RAM, ~60 MB model).
  3. Standardized on 768 dimensions, cutting PostgreSQL HNSW index RAM usage by 50% compared to 1536-dimensional models.

### 2026-09-25 — OKF v0.2 Implementation & Obsidian Integration
* **Actors:** `[antigravity]`, reviewed by `human:aprxty3`
* **Context:** Standardizing persistent knowledge management across coding agents.
* **Decision:**
  1. Standardized [knowledge/index.md](knowledge/index.md) with 4-tier progressive disclosure.
  2. Published persistent agent rules in `.agents/rules/` and `.agents/workflows/`.
  3. Extracted codebase knowledge graph into `graphify-out/` and synchronized `obsidian-vault/`.

### 2026-09-25 — 6 Core Engineering Pillars & Zero Emoji Policy
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Establishing guardrails against technical debt and informal slop.
* **Decision:**
  1. Codified 6 engineering invariants:
     - ROBUST: Zero panics in production Rust (`Result<T, AppError>`).
     - SCALABLE: Stateless Axum, scoped vector queries (`WHERE book_id = $1`).
     - EASY TO MAINTAIN: Isolated modular crates and paired SQL migrations.
     - DRY: Shared DTOs and validation in `crates/shared`.
     - KISS: Postgres for Everything, Redis Streams for queues.
     - YAGNI: Strict MVP focus; no speculative abstractions.
  2. Enforced Zero Emoji Policy across all code, commits, and documentation.

### 2026-09-25 — Database Selection: "Postgres for Everything" vs Elasticsearch
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Evaluating catalog discovery and semantic search requirements.
* **Decision:**
  1. Rejected Elasticsearch to avoid 2–4 GB JVM RAM overhead and dual-write synchronizations.
  2. Chose PostgreSQL 17 FTS + `pg_trgm` GIN indexes for typo-tolerant catalog search (<3ms).
  3. Chose `pgvector` HNSW indexes for scoped quote search (<10ms).

### 2026-09-25 — Queue Architecture: Redis Streams vs RedPanda / RabbitMQ
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Asynchronous queue for EPUB ingestion and transactional emails.
* **Decision:** Used Redis Streams within existing Redis 7 infrastructure, rejecting external brokers to maintain simplicity.

### 2026-09-25 — Data Access: SeaORM & Paired SQL Migrations
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Type-safe database queries and auditable schema evolutions.
* **Decision:**
  1. Adopted SeaORM for async ORM and query building.
  2. Managed database schema via paired SQL scripts (`.up.sql` and `.down.sql`).
  3. Centralized lifecycle automation in `Makefile`.

### 2026-09-25 — Visual Identity: Vintage Literary (1900–1950)
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Distinctive visual aesthetics for public domain literature.
* **Decision:**
  1. Aged paper palette (`#F9F6F0`), deep ink (`#2B2625`), terracotta accent (`#9D5A3C`), and classic serif typography.
  2. Integrated atomic insight cards and chapter recaps for complex classical texts.

### 2026-09-26 — Monorepo Initialization & 5-Crate Workspace
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Modular foundation ensuring DRY principles between backend and WASM.
* **Decision:**
  1. Cargo workspace resolver v2 with centralized dependencies.
  2. Five focused crates: `shared` (platform-agnostic DTOs), `domain` (business models), `infra` (connections), `server` (Axum REST API), `web` (Leptos WASM).
  3. Automation managed via `Makefile`.

### 2026-09-26 — Database Migration Execution & 16-Index Schema Verification
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Implementing physical PostgreSQL 17 schema and indexes from ERD specifications.
* **Decision:**
  1. Applied 6 paired timestamped migration files covering 14 tables and 37 indexes.
  2. Verified partial GIN FTS and Trigram indexes (`WHERE status = 'published'`) and HNSW vector index.
  3. Switched MinIO container image to Chainguard and mapped to host ports 9005/9006.

### 2026-09-26 — Milestone Breakdown & Task Governance (`knowledge/tasks/`)
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Structured sequential implementation roadmap toward MVP.
* **Decision:** Structured 7 sequential milestones in `knowledge/tasks/` linked to PRD user stories, FRD modules, and SRS API endpoints.

### 2026-09-26 — OpenAPI Utoipa, Hot Reload, Structured Logging & 4-Tier Test Suite
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Interactive API documentation, rapid developer turnaround, and robust verification.
* **Decision:**
  1. Integrated `utoipa` and `utoipa-swagger-ui` (OpenAPI 3.1) at compile time. Isolated behind `openapi` feature in `crates/shared` to avoid WASM bloat.
  2. Standardized hot-reload commands: `make dev-server` via `cargo-watch` and `make dev-web` via Trunk.
  3. Structured logging with NDJSON mode (`LOG_FORMAT=json`) and automatic `x-request-id` propagation.
  4. Built comprehensive 4-tier test suite in `crates/server/tests/`: smoke, integration, performance SLA (p95 < 50ms), and reliability tests using `mockall`, `claims`, `pretty_assertions`, and `rstest`.

### 2026-09-26 — Infrastructure Layer: Modular AppConfig, Dynamic Pools & 13 SeaORM Entities (ADR-14)
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Strongly-typed configuration, safe resource allocation, and type-safe relational/vector queries across all 13 PostgreSQL tables.
* **Decision:**
  1. Standardized `dotenvy = "0.15"` in root workspace and `crates/infra` for zero-panic environment loading with fallback defaults.
  2. Implemented modular configurations: `ServerConfig`, `DatabaseConfig`, `RedisConfig`, `AuthConfig`, `StorageConfig`, `EmailConfig`, and `AiConfig`.
  3. Dynamic PostgreSQL pool creation using `ConnectOptions` (configurable max/min connections, connect/idle timeouts) and Redis client initialization in `crates/infra/src/pool.rs`.
  4. Modeled all 13 database tables into SeaORM entities in `crates/infra/src/entities/`, enabling `postgres-vector` (`PgVector`) for `book_chunks.embedding` and `Decimal` for reading completion percentages.
  5. Refactored `AppState` to hold `Arc<AppConfig>` to avoid deep clones across Axum requests.

### 2026-09-26 — Authentication, JWT/Argon2id Security, Redis OTP & Guest Reconciliation (ADR-15)
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Secure user registration, authentication, token rotation, guest reading synchronization, and RBAC protection according to SRS 1–7 and FRD Module 3.
* **Decision:**
  1. Implemented Argon2id password hashing (64MB memory, 3 iterations, 4 parallelism) and verification via `argon2 = "0.5"`.
  2. Generates 6-digit numeric OTPs via `OsRng`, storing SHA-256 hashed digest in Redis (`otp:{email}`) with 10-minute TTL and 3-attempt limit. Integrated async SMTP email delivery targeting Mailpit with fallback logging.
  3. Configured JWT access tokens (24-hour TTL, HS256) and cryptographically secure 32-byte refresh tokens stored in Redis (`refresh_token:{token}`) with 14-day TTL. Implemented single-use refresh token rotation on `/api/v1/auth/refresh` and token revocation on logout.
  4. Implemented `AuthUser` Axum extractor, `require_admin` RBAC middleware, and Redis sliding-window rate limiting (20 req/min for auth endpoints).
  5. Implemented guest progress reconciliation (`/api/v1/progress/merge`) executing SQL upsert with `GREATEST` progress resolution without data loss.
  6. Verified complete auth lifecycle and guest progress merge with integration tests achieving 100% pass rate.

### 2026-09-26 — Catalog Backend, Typo-Tolerant FTS, Reader Engine & Gamification (ADR-16)
* **Actors:** `human:aprxty3` & `[antigravity]`
* **Context:** Public domain book discovery, typo-tolerant search (<3ms), chapter content delivery, offline bundles, CFI reading progress persistence, and daily streak gamification.
* **Decision:**
  1. Built catalog repository with cursor-based pagination and multi-dimensional filters (language, theme, tags).
  2. Implemented typo-tolerant lexical search combining `pg_trgm` `word_similarity` / `%>` operators with `tsvector` FTS matching (<3ms).
  3. Built chapter delivery and offline bundle retrieval (`/api/v1/books/{id}/offline-bundle`) bundling metadata and XHTML content for client-side IndexedDB caching.
  4. Implemented reading progress synchronization with EPUB CFI anchors and auto-completion when progress reaches 100%.
  5. Implemented daily streak engine: logs reading heartbeats, tracks 5-minute daily threshold, calculates consecutive streaks, awards XP, and unlocks milestone badges.
  6. Created modular manual testing guides in `knowledge/manual-test/` (tasks 01–03).
  7. Verified 100% pass rate across workspace test suite (32 tests).



