# CLAUDE.md — Claude Code Guidelines for Project Baca

This document defines build commands, architecture patterns, and coding invariants for Claude Code operating on the **Project Baca** monorepo.

---

## 1. Primary Commands (Makefile Orchestration)

All lifecycle tasks must be executed via `Makefile` targets:

```bash
# Infrastructure Lifecycle
make db-up            # Start PostgreSQL 17, Redis 7, MinIO, Mailpit via docker-compose
make db-down          # Stop all local infrastructure services
make db-logs          # View aggregate infrastructure logs

# Database Migrations (PostgreSQL 17)
make migrate-up       # Apply pending SQL migrations from migrations/*.up.sql
make migrate-down     # Revert last SQL migration using migrations/*.down.sql
make migrate-reset    # Revert all migrations and re-apply from scratch

# Build & Verification
make check            # Run cargo clippy, cargo fmt --check, and ruff lint
make test             # Run Rust unit/integration tests and python worker tests
make build            # Build server release binary and Leptos WASM bundle

# Local Development Server
make dev              # Run Axum backend, Trunk WASM dev server, and worker concurrently
```

---

## 2. Monorepo Architecture Overview

Project Baca is a polyglot monorepo with strict domain boundaries:

* `crates/domain/`: Pure domain types, entities, Newtype IDs (`BookId`, `UserId`), and invariants. No external I/O dependencies.
* `crates/shared/`: Shared DTOs, API payloads, validation logic, and bidirectional i18n dictionaries (`ID` / `EN`).
* `crates/infra/`: Infrastructure adapters (SeaORM repositories, Redis connection pool, MinIO S3 client).
* `crates/server/`: Axum HTTP server, JWT & Argon2id auth handlers, Redis sliding-window rate limiters, REST API endpoints.
* `crates/web/`: Leptos 0.7 WASM frontend, reactive state, reflowable multi-column book reader, offline `IndexedDB` caching.
* `python_worker/`: Python ingestion service for EPUB unzipping, HTML sanitization, scene-based chunking, and Dual-Mode Embedding client (Gemini API & FastEmbed CPU 768-dim).
* `knowledge/`: Google Cloud Open Knowledge Format (OKF v0.2) specifications SSOT.

---

## 3. Strict Coding Invariants

1. **Zero Panics in Production Code:**
   - Never call `.unwrap()` or `.expect()` in `crates/server`, `crates/domain`, `crates/infra`, `crates/shared`, or `crates/web`.
   - Propagate all errors via `Result<T, AppError>`.
2. **Postgres for Everything (No Elasticsearch):**
   - Lexical catalog search: PostgreSQL 17 FTS + `pg_trgm` GIN index (<3ms).
   - Scoped semantic quote search: `pgvector` HNSW index (<10ms) isolated per book (`WHERE book_id = $1`).
3. **Redis Streams for Async Tasks (No Dedicated Message Broker):**
   - Ingestion jobs and transactional email dispatch are queued on Redis Streams.
4. **Vintage Literary Aesthetic:**
   - Antarmuka web Leptos WASM menggunakan palet aged paper, tinta iron gall, dan tipografi serif sastrawan (merujuk `knowledge/prd.md`).
5. **Zero Emoji Policy:**
   - Absolutely no emojis, emoticon glyphs, or informal tone in code, comments, docstrings, commit messages, or markdown documentation.
6. **Core Software Engineering Principles:**
   - Adhere to **ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, and YAGNI** on every modification.

---

## 4. Documentation & Audit Trail Workflow

1. Always consult [knowledge/index.md](knowledge/index.md) before designing architectural changes.
2. Record all significant decisions and code changes in [knowledge/log.md](knowledge/log.md) and [MEMORY.md](MEMORY.md).
3. Update [PROJECT_LOG.md](PROJECT_LOG.md) to maintain task tracking.
4. Run `graphify cluster-only` or `graphify update .` to keep the structural knowledge graph current.
