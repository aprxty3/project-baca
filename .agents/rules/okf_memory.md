---
trigger: always_on
description: Open Knowledge Format (OKF v0.2) & Persistent Memory Guidelines for Project Baca.
---

## Open Knowledge Format (OKF) & Persistent Agent Rules

This workspace follows the **Google Cloud Open Knowledge Format (OKF v0.2)** specification and Graphify knowledge graphs as the Single Source of Truth (SSOT).

### 1. OKF Progressive Disclosure
- Consult [knowledge/index.md](knowledge/index.md) before inspecting raw code or making architectural decisions.
- Use Graphify tools (`graphify query "<question>"`, `query_graph`, `shortest_path`, `get_node`) for relationship navigation.
- Progressive disclosure hierarchy:
  - **Layer 1 (Orientation):** [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md), [knowledge/index.md](knowledge/index.md).
  - **Layer 2 (Requirements):** [knowledge/prd.md](knowledge/prd.md), [knowledge/ux-flow.md](knowledge/ux-flow.md).
  - **Layer 3 (Technical Specs):** [knowledge/frd.md](knowledge/frd.md), [knowledge/erd.md](knowledge/erd.md), [knowledge/srs.md](knowledge/srs.md).
  - **Layer 4 (Audit Trail):** [knowledge/log.md](knowledge/log.md), [PROJECT_LOG.md](PROJECT_LOG.md).

### 2. Audit Trail Protocol
- After meaningful changes:
  1. Record entries in [knowledge/log.md](knowledge/log.md) (actor, date, action, affected concepts).
  2. Update [PROJECT_LOG.md](PROJECT_LOG.md).
  3. Run `graphify update .` or `graphify cluster-only`.

### 3. Factual Verification
- Never guess crate parameters, APIs, or database syntax. Verify using official documentation or search tools.

### 4. Non-Negotiable Core Invariants
- **Zero Panics in Rust:** No `unwrap()` or `expect()` in production code paths. Use `Result<T, AppError>`.
- **Postgres for Everything:** Lexical FTS + `pg_trgm` GIN index (<3ms). Semantic `pgvector` HNSW index (<10ms). No Elasticsearch.
- **Pragmatic Broker:** Redis Streams for ingestion and emails. No separate brokers (RedPanda/RabbitMQ) for MVP.
- **Paired SQL Migrations:** Schema changes managed via `<timestamp>_<name>.up.sql` and `<timestamp>_<name>.down.sql` in `migrations/`.
- **Centralized Automation:** All tasks accessible via `Makefile`.
- **Vintage Literary Aesthetic:** Leptos WASM frontend reflects aged paper, dark ink, and literary serif typography.

### 5. Engineering Principles
- **ROBUST:** Graceful degradation on all failures.
- **SCALABLE:** Stateless Axum API, scoped semantic queries (`WHERE book_id = $1`), partial GIN indexing.
- **EASY TO MAINTAIN:** Clear crate boundaries (`domain`, `shared`, `infra`, `server`, `web`).
- **DRY:** Shared DTOs and validation rules in `crates/shared`.
- **KISS:** Simple, reliable solutions without artificial abstraction layers.
- **YAGNI:** Strict MVP scope. Defer future features.

### 6. Testing Protocol
- Validate via: Smoke tests, Unit tests, and Integration tests.
- **SLA Targets:** REST API p95 < 50ms, Lexical FTS < 5ms, Scoped Quote Search < 10ms, WASM Reader 60 FPS.

### 7. Zero Emoji Policy & Professional Tone
- Strictly no emojis or graphical emoticons in code, comments, commit messages, or documentation.

### 8. Repository Link Integrity
- Never use local absolute paths. Always use clean relative markdown links (e.g., `knowledge/prd.md`).
