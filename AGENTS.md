---
type: Agent Guidelines
title: "Project Baca — AI Agent Guidelines & Governance"
description: "Operational instructions, architectural boundaries, and execution protocols for AI coding agents."
tags: [agents, ai-instructions, conventions, governance, okf]
---

# AGENTS.md — AI Agent Guidelines & Governance

Standard operating rules for all AI coding agents working on **Project Baca**.

## 1. Single Source of Truth (SSOT)

1. **OKF v0.2 Knowledge Catalog:**
   - The `knowledge/` directory is the canonical source of truth for all product and technical specifications.
   - Always consult [knowledge/index.md](knowledge/index.md) before making architectural decisions or writing code.
2. **Graphify Knowledge Graph:**
   - Use `graphify query "<question>"`, `shortest_path`, or `get_node` instead of broad file scans.
   - Inspect community clusters in [graphify-out/GRAPH_REPORT.md](graphify-out/GRAPH_REPORT.md).

## 2. Non-Negotiable Core Invariants

1. **Zero Panics in Rust:**
   - `unwrap()` and `expect()` are prohibited in production paths (`crates/server`, `crates/domain`, `crates/infra`, `crates/shared`, `crates/web`).
   - All errors must be handled cleanly via `Result<T, AppError>`.
2. **Postgres for Everything:**
   - Lexical catalog search via PostgreSQL 17 FTS + `pg_trgm` GIN indexes (<3ms).
   - Semantic quote search via `pgvector` HNSW indexes (<10ms).
   - No Elasticsearch.
3. **Pragmatic Message Broker:**
   - Redis Streams for EPUB ingestion and transactional email queues.
   - No dedicated brokers (RedPanda/RabbitMQ) for MVP.
4. **Paired SQL Migrations:**
   - All schema changes managed in `migrations/` with paired scripts: `<timestamp>_<name>.up.sql` and `<timestamp>_<name>.down.sql`.
5. **Centralized Makefile Automation:**
   - All build, test, run, and migration commands must be accessible via `Makefile`.
6. **Vintage Literary Aesthetic (1900–1950):**
   - The Leptos WASM frontend reflects classic print aesthetics (aged paper `#F9F6F0`, dense ink `#2B2625`, terracotta `#9D5A3C`, serif typography).

## 3. Engineering Pillars

* **ROBUST:** Handle all failures gracefully without crashing.
* **SCALABLE:** Stateless Axum API, scoped semantic search (`WHERE book_id = $1`), and partial GIN indexes (`WHERE status = 'published'`).
* **EASY TO MAINTAIN:** Clear crate boundaries (`domain`, `shared`, `infra`, `server`, `web`), structured OKF documentation, modular code.
* **DRY:** Shared DTOs and validation rules in `crates/shared`.
* **KISS:** Choose simple, reliable solutions without unnecessary layers of abstraction.
* **YAGNI:** Implement only specifications required for MVP. Defer future features.

## 4. Audit Trail Protocol

Upon completing meaningful code changes, refactors, or schema migrations:
1. Log new entries in [knowledge/log.md](knowledge/log.md) (actor, date, actions, affected concepts).
2. Record architectural decisions in [MEMORY.md](MEMORY.md).
3. Update operational status in [PROJECT_LOG.md](PROJECT_LOG.md).
4. Run `graphify update .` or `graphify cluster-only`.

## 5. Zero Emoji Policy & Link Integrity

* **Zero Emoji Policy:** Strictly no emojis or graphical emoticons in code, comments, docstrings, commit messages, or technical documentation.
* **Repository Link Integrity:** Never use local absolute paths. Always use clean relative markdown links (e.g., `knowledge/prd.md`).
