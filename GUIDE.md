---
type: Developer Guide
title: "Project Baca — Developer Guide & Local Operations"
description: "Step-by-step instructions for local development, database migrations, WASM compilation, and test execution."
tags: [guide, onboarding, developer, setup, workflow, okf]
---

# GUIDE.md — Developer & Operations Guide

Practical instructions to build, run, test, and develop **Project Baca** locally.

## 1. Prerequisites

1. **Rust Toolchain:**
   - Rust 1.80+ (stable).
   - WASM target: `rustup target add wasm32-unknown-unknown`.
   - Tooling: `rustup component add clippy rustfmt`.
2. **WASM Bundler:**
   - Trunk: `cargo install trunk` (or via OS package manager).
3. **Container Runtime:**
   - Docker & Docker Compose (v2.20+).
4. **Python Environment (for Ingestion Worker):**
   - Python 3.11+ with `uv` or `venv`.

## 2. Infrastructure Services

Orchestrate local dependencies via Docker Compose:

```bash
# Start background containers
make db-up

# Check container health
docker compose ps
```

### Port Mapping
* **PostgreSQL 17 + pgvector:** `localhost:5433` (`baca_user` / `project_baca_db`)
* **Redis 7 (Streams & Cache):** `localhost:6380`
* **MinIO Object Storage:** S3 API on `localhost:9005`, Console on `http://localhost:9006`
* **Mailpit (SMTP):** SMTP on `localhost:1025`, Web UI on `http://localhost:8025`

## 3. Database Migrations

Manage database schema via paired SQL files in `migrations/`:

```bash
make migrate-up     # Apply pending migrations
make migrate-down   # Rollback latest migration
make migrate-reset  # Full reset and re-apply
```

## 4. Running Application Components

### Development Commands
* **`make dev`:** Orchestrator overview showing running ports and service commands.
* **`make dev-server`:** Runs Axum backend with auto-reload via `cargo-watch` (port 8080).
* **`make dev-web`:** Runs Leptos WASM frontend with hot-reload via Trunk (port 3000).

### API Documentation & Swagger UI
* **Swagger UI:** Interactive testing UI at `http://localhost:8080/swagger-ui`.
* **OpenAPI Spec:** Raw JSON spec at `http://localhost:8080/api-docs/openapi.json`.
* Schema definitions are compiled directly from shared DTOs in `crates/shared`.

### Structured Logging & Observability
* **Human-readable text:** Default output for local development.
* **Structured JSON:** Enable via `LOG_FORMAT=json cargo run -p server`.
* **Request Correlation:** The `request_id_middleware` assigns or propagates `x-request-id` headers across all spans and responses.

### Ingestion Worker (Python)
```bash
cd python_worker
uv venv && source .venv/bin/activate
uv pip install -r requirements.txt
python -m ingestion.worker
```

## 5. Testing & Quality Assurance

Comprehensive 4-tier test suite:

```bash
make test-smoke        # Service boot, health endpoints, Swagger UI, network reachability
make test-integration  # Request ID propagation, CORS, OpenAPI schema verification
make test-performance  # Latency SLA benchmarks (p95 < 50ms, 50 concurrent tasks)
make test-reliability  # Database disconnect fallback, fault injection, zero-panic checks
make test-unit         # Domain models and DTO validation logic
make test-all          # Complete end-to-end test execution
```

### Test Libraries
* **Mocking:** `mockall` (declarative mock generation for trait ports).
* **Assertions:** `pretty_assertions` (colored diffs) and `claims` (`assert_ok!`, `assert_err!`).
* **Parameterized Tests:** `rstest` (fixtures and table-driven test cases).

## 6. Troubleshooting

1. **Port 5433 Conflict:** Check active processes with `lsof -i :5433` and adjust `docker-compose.yml` if necessary.
2. **Missing WASM Target:** Run `rustup target add wasm32-unknown-unknown`.
3. **MinIO Connection Error:** Ensure buckets (`project-baca-books`) are created during container startup.
4. **Redis Streams Group:** The worker auto-initializes the consumer group using `XGROUP CREATE` fallback.
