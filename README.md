# Project Baca

Modern e-reader combining distraction-free, reflowable reading with atomic insight cards and semantic quote discovery. Built as an offline-first, open-access platform for curated public domain literature.

## 1. Architecture & Tech Stack

### Production Runtime
* **Backend API:** [Rust](https://www.rust-lang.org/) — [Axum](https://github.com/tokio-rs/axum) (REST API, Auth, OpenAPI / Swagger UI).
* **Frontend Web PWA:** [Leptos 0.7](https://leptos.dev/) (WASM) — Reflowable pagination, IndexedDB offline storage, and reactive language switching.
* **AI Ingestion Worker:** Python — EPUB parsing, scene chunking, and Google GenAI SDK integration.
* **Vector Engine:** Dual-mode embeddings via Google GenAI (768-dim) or local CPU FastEmbed ONNX.
* **Database & Search:** PostgreSQL 17 + [`pgvector`](https://github.com/pgvector/pgvector) — Scoped HNSW index per book, `pg_trgm`, and Full-Text Search.
* **Object Storage:** S3-compatible (MinIO for local dev, Cloudflare R2 for production).
* **Cache & Queues:** Redis 7 — Session cache, rate limiting, and Redis Streams for ingestion.
* **Transactional Email:** SMTP (Mailpit for dev, Resend/SES for production).
* **Visual Aesthetics:** Mid-Century Vintage Literary (1900–1950) editorial theme with aged paper, deep espresso tones, and terracotta accents.

### Agentic Intelligence System
* **Layer 0 (Jev / Laya):** Fast reflex gate (<0.2ms) for intent routing and shell safety guardrails.
* **Layer 1 (Graphify):** Codebase AST and knowledge graph navigation via `graphify-out/`.
* **Layer 2 (OKF Vault v0.2):** Canonical specifications in `knowledge/` serving as single source of truth (SSOT).
* **Layer 3 (GBrain):** Cross-session persistent memory in PostgreSQL 17 pgvector.

## 2. Documentation Directory

### Core Specifications (OKF v0.2)
* **Master Index:** [knowledge/index.md](knowledge/index.md)
* **Product Requirements (PRD):** [knowledge/prd.md](knowledge/prd.md)
* **Functional Requirements (FRD):** [knowledge/frd.md](knowledge/frd.md)
* **Database Schema & Indexes (ERD):** [knowledge/erd.md](knowledge/erd.md)
* **User Flows & Wireframes (UX Flow):** [knowledge/ux-flow.md](knowledge/ux-flow.md)
* **System Requirements & API Contracts (SRS):** [knowledge/srs.md](knowledge/srs.md)
* **Specification Audit Trail:** [knowledge/log.md](knowledge/log.md)

### Architecture & Operations
* **Monorepo Blueprint:** [ARCHITECTURE.md](ARCHITECTURE.md)
* **Distributed Processing & Workers:** [DISTRIBUTED.md](DISTRIBUTED.md)
* **Architecture Decision Records (ADR):** [MEMORY.md](MEMORY.md)
* **Visual Asset Catalog:** [assets/README.md](assets/README.md)
* **Legal & Public Domain Statement:** [NOTICE.md](NOTICE.md)
* **Release History & Milestones:** [CHANGELOG.md](CHANGELOG.md)
* **Sprint Backlog & Operational Status:** [PROJECT_LOG.md](PROJECT_LOG.md)

### AI Agent Governance
* **General AI Coding Agent Guidelines:** [AGENTS.md](AGENTS.md)
* **Claude Code Instructions:** [CLAUDE.md](CLAUDE.md)
* **Google Antigravity & Gemini Instructions:** [GEMINI.md](GEMINI.md)

## 3. Quickstart

Detailed step-by-step setup is documented in [GUIDE.md](GUIDE.md).

```bash
# 1. Start core infrastructure (PostgreSQL 17, Redis 7, MinIO, Mailpit)
make db-up

# 2. Run backend API with hot reload
make dev-server

# 3. Run frontend Web Reader with hot reload
make dev-web
```
