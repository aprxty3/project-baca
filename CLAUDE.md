# CLAUDE.md — Claude Code Guidelines for Project Baca

Build commands, architecture patterns, and coding invariants for Claude Code operating on **Project Baca**.

## 1. Primary Commands (Makefile Orchestration)

Execute all lifecycle tasks via `Makefile` targets:

```bash
# Infrastructure
make db-up            # Start PostgreSQL 17, Redis 7, MinIO, Mailpit
make db-down          # Stop infrastructure containers
make db-logs          # View container logs

# Database Migrations
make migrate-up       # Apply pending migrations
make migrate-down     # Revert last migration
make migrate-reset    # Re-apply all migrations

# Development & Testing
make dev-server       # Axum backend with hot reload
make dev-web          # Leptos WASM frontend with hot reload
make check            # Run clippy and fmt checks
make test-all         # Execute 4-tier test suite
```

## 2. Monorepo Architecture Overview

* `crates/domain/`: Pure domain types, entities, Newtype IDs (`BookId`, `UserId`). No external I/O.
* `crates/shared/`: Shared DTOs, API payloads, validation logic, and i18n dictionaries.
* `crates/infra/`: Infrastructure adapters (SeaORM repositories, Redis pool, MinIO client).
* `crates/server/`: Axum HTTP server, JWT/Argon2id auth, OpenAPI / Swagger UI, rate limiting.
* `crates/web/`: Leptos 0.7 WASM frontend, reflowable reader, IndexedDB offline caching.
* `python_worker/`: EPUB extraction, HTML sanitization, scene chunking, Dual-Mode Embeddings.
* `knowledge/`: Canonical OKF v0.2 specifications SSOT.

## 3. Strict Coding Invariants

1. **Zero Panics in Production Code:**
   - Prohibited `.unwrap()` or `.expect()` in `crates/server`, `crates/domain`, `crates/infra`, `crates/shared`, `crates/web`.
   - Propagate all errors via `Result<T, AppError>`.
2. **Postgres for Everything (No Elasticsearch):**
   - Lexical catalog search: PostgreSQL 17 FTS + `pg_trgm` GIN index (<3ms).
   - Scoped semantic quote search: `pgvector` HNSW index (<10ms) scoped per book (`WHERE book_id = $1`).
3. **Redis Streams for Async Tasks (No Dedicated Message Broker):**
   - Ingestion jobs and transactional email queues run on Redis Streams.
4. **Vintage Literary Aesthetic:**
   - Leptos WASM frontend reflects aged paper, dark ink, and literary serif typography.
5. **Zero Emoji Policy:**
   - Absolutely no emojis or graphical emoticons in code, comments, commit messages, or documentation.
6. **Core Engineering Principles:**
   - Adhere strictly to ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, and YAGNI.

## 4. Documentation & Audit Trail Workflow

1. Consult [knowledge/index.md](knowledge/index.md) before designing architectural changes.
2. Record significant decisions and code changes in [knowledge/log.md](knowledge/log.md) and [MEMORY.md](MEMORY.md).
3. Update [PROJECT_LOG.md](PROJECT_LOG.md) for operational backlog tracking.
4. Run `graphify update .` to keep the knowledge graph synchronized.
