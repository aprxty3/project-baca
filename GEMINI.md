---
type: Agent Guidelines
title: "Project Baca — Gemini & Antigravity Instructions"
description: "Standard operational instructions for Google Antigravity and Gemini models operating in Project Baca."
tags: [gemini, antigravity, guidelines, instructions, okf]
---

# GEMINI.md — Gemini & Google Antigravity Guidelines

Operational instructions for **Google Antigravity** and **Gemini** models in **Project Baca**.

## 1. Navigation & Source of Truth (SSOT)

* **OKF v0.2 Master Catalog:** Consult [knowledge/index.md](knowledge/index.md) before inspecting raw code or making architectural decisions.
* **Graphify Knowledge Graph:** Use `query_graph`, `shortest_path`, or `graphify query` for relationship navigation in `graphify-out/`.
* **Progressive Disclosure:**
  - Layer 1: [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md), [knowledge/index.md](knowledge/index.md).
  - Layer 2: [knowledge/prd.md](knowledge/prd.md), [knowledge/ux-flow.md](knowledge/ux-flow.md).
  - Layer 3: [knowledge/frd.md](knowledge/frd.md), [knowledge/erd.md](knowledge/erd.md), [knowledge/srs.md](knowledge/srs.md).
  - Layer 4: [knowledge/log.md](knowledge/log.md), [MEMORY.md](MEMORY.md), [PROJECT_LOG.md](PROJECT_LOG.md).

## 2. Non-Negotiable Engineering Invariants

1. **Rust Zero Panics:**
   - No `.unwrap()` or `.expect()` in production code paths.
   - Use `Result<T, AppError>` for all error propagation.
2. **Postgres for Everything:**
   - Lexical search: PostgreSQL 17 FTS + `pg_trgm` GIN index (<3ms).
   - Semantic search: PostgreSQL 17 `pgvector` HNSW index (<10ms).
   - No Elasticsearch.
3. **Pragmatic Message Broker:**
   - Redis Streams for EPUB ingestion and transactional email queues. No RabbitMQ or RedPanda for MVP.
4. **SeaORM & Paired SQL Migrations:**
   - Schema managed via paired `.up.sql` and `.down.sql` scripts in `migrations/`.
   - Automation driven through `Makefile`.
5. **Vintage Literary Aesthetic (1900–1950):**
   - Leptos WASM frontend uses aged paper palette, classic ink, terracotta accents, and literary serif typography.

## 3. Engineering Discipline & Zero Emoji Policy

* **ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, YAGNI:** Mandatory on every edit.
* **Zero Emoji Policy:** Strictly no emojis or graphical emoticons in code, comments, docstrings, commit messages, or documentation.
* **Audit Trail Protocol:** Record updates in [knowledge/log.md](knowledge/log.md) and [MEMORY.md](MEMORY.md), then synchronize graph via `graphify update .`.
