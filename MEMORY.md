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

