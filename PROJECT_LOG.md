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
2. **[ACTIVE] Milestone 01 — Infrastructure, Modular AppConfig & SeaORM Entities:**
   - Spec: [knowledge/tasks/01_infrastructure_and_config.md](knowledge/tasks/01_infrastructure_and_config.md)
   - Scope: `dotenvy` integration, modular configurations, dynamic SeaORM pool, 13 SeaORM entity definitions.
3. **[PENDING] Milestone 02 — Authentication, JWT/Argon2id, OTP & Guest Reconciliation:**
   - Spec: [knowledge/tasks/02_authentication_and_user.md](knowledge/tasks/02_authentication_and_user.md)
   - Scope: Auth endpoints, Argon2id hashing, Redis OTP, token rotation, guest merging, RBAC guards.
4. **[PENDING] Milestone 03 — Catalog Backend, Trigram FTS, Reader API & Streak Gamification:**
   - Spec: [knowledge/tasks/03_catalog_and_reader_backend.md](knowledge/tasks/03_catalog_and_reader_backend.md)
   - Scope: Cursor-based catalog, Trigram FTS (<3ms), chapter content, progress sync, streak heartbeats.
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
