# Changelog — Project Baca

All notable changes are documented chronologically following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] — 2026-09-25

### Added
- **Open Knowledge Format (OKF v0.2):** Full SSOT specification vault in `knowledge/` (`prd.md`, `erd.md`, `ux-flow.md`, `frd.md`, `srs.md`, `index.md`, `log.md`).
- **Graphify Knowledge Graph:** Codebase graph extraction (nodes, edges, communities) in `graphify-out/`.
- **Obsidian Vault Export:** Structured markdown vault notes in `obsidian-vault/`.
- **Workspace Agent Rules:** Agent governance in `.agents/rules/okf_memory.md`, `.agents/rules/graphify.md`, and `.agents/workflows/graphify.md`.
- **Multi-Agent Guidelines:** Added [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md), [MEMORY.md](MEMORY.md), [GEMINI.md](GEMINI.md), [GUIDE.md](GUIDE.md), [DISTRIBUTED.md](DISTRIBUTED.md), [NOTICE.md](NOTICE.md).
- **System Specifications:** 23 REST API contracts, multi-layer security model in `knowledge/srs.md`.
- **Database Schema & Index Matrix:** 16 PostgreSQL 17 indexes including partial GIN and HNSW in `knowledge/erd.md`.
- **UX Flows & Wireframes:** Navigation layout and guest-to-cloud sync flows in `knowledge/ux-flow.md`.
- **Functional Requirements:** Specifications across 7 system modules in `knowledge/frd.md`.
- **OpenAPI & Swagger UI:** Integrated `utoipa` with Swagger UI at `/swagger-ui` and raw JSON spec at `/api-docs/openapi.json`.
- **4-Tier Test Suite:** Smoke, integration, performance SLA (p95 < 50ms), and reliability tests with `mockall`, `claims`, `pretty_assertions`, and `rstest`.

### Changed
- **Embedding Engine:** Standardized on 768-dim Dual-Mode Provider (Google GenAI API and FastEmbed CPU ONNX), eliminating external Triton inference servers and halving HNSW memory usage.
- **Search Strategy:** Adopted PostgreSQL 17 FTS + `pg_trgm` GIN indexes (<3ms) in place of Elasticsearch to save memory and eliminate dual-write overhead.
- **Message Broker:** Adopted Redis Streams as the single queue for ingestion and email delivery.
- **Style Policy:** Enforced Zero Emoji Policy across all documentation and codebase.
- **Visual Theme:** Standardized on Vintage Literary (1900–1950) editorial theme with reactive bilingual switching `[ID|EN]`.

### Security
- Defense-in-depth architecture: Cloudflare edge rate limits, Redis sliding-window limits, SameSite CSRF cookies, Argon2id password hashing, and SHA-256 hashed 6-digit OTPs.
- Environment isolation: Added `.env.example` and gitignore rules for private specifications.

## [0.1.0] — 2026-09-25

### Added
- **Project Initialization:** Created monorepo blueprint [ARCHITECTURE.md](ARCHITECTURE.md) and [README.md](README.md).
- **Local Infrastructure:** Configured [docker-compose.yml](docker-compose.yml) for PostgreSQL 17 + pgvector, Redis 7, MinIO, and Mailpit.
- **Legal Copyright Analysis:** Verified public domain copyright compliance (life of author + 70 years under Indonesian Law 28/2014 and international terms).
- **Visual Asset Catalog:** Cataloged 5 vintage pen-and-ink illustrations in `assets/README.md`.
