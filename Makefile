-include .env
export

DB_USER ?= baca_user
DB_NAME ?= project_baca_db

CARGO_WATCH ?= $(shell which cargo-watch 2>/dev/null)
TRUNK ?= $(shell which trunk 2>/dev/null)

# Starts the development environment
dev:
	@echo "Project Baca — Development Environment"
	@echo "Gunakan 'make dev-server' untuk backend (Live-Reload seperti Air di Go)"
	@echo "Gunakan 'make dev-web' untuk frontend WASM (Trunk Hot-Reload)"
	@echo "Gunakan 'make db-up' untuk menyalakan database & infrastruktur"

# Starts backend with live reload (like Air in Go)
dev-server:
	@echo "Starting backend development server..."
ifeq ($(strip $(CARGO_WATCH)),)
	@echo "Notice: cargo-watch belum terpasang. Menjalankan 'cargo run -p server' biasa."
	@echo "Tip: Pasang cargo-watch untuk auto-reload seperti Air (sudo pacman -S cargo-watch atau cargo install cargo-watch)."
	@cargo run -p server
else
	@echo "Live reload aktif via cargo-watch..."
	@cargo watch -q -c -w crates/server -w crates/infra -w crates/domain -w crates/shared -x "run -p server"
endif

# Starts frontend with Trunk native hot-reload
dev-web:
	@echo "Starting frontend development server (Trunk Hot-Reload)..."
	@cd crates/web && trunk serve

# Starts all infrastructure containers in detached mode
db-up:
	@echo "Starting database & infrastructure services..."
	@docker compose up -d

# Stops all infrastructure containers
db-down:
	@echo "Stopping infrastructure services..."
	@docker compose down

# Stop and remove all containers, networks, and volumes (WARNING: DB data will be lost)
db-prune:
	@echo "Stopping and removing all containers and volumes (WARNING: DB data will be lost)..."
	@docker compose down -v

# View container logs
db-logs:
	@docker compose logs -f

# Connect to the psql shell inside the 'postgres' container
db-shell:
	@docker compose exec postgres psql -U $(DB_USER) -d $(DB_NAME)

# Connect to the redis-cli shell inside the 'redis' container
redis-shell:
	@docker compose exec redis redis-cli

# Database migrations
migrate-up:
	@echo "Menjalankan migrasi basis data (up)..."
	@./scripts/migrate.sh up

migrate-down:
	@echo "Membalikkan migrasi basis data terakhir (down)..."
	@./scripts/migrate.sh down

migrate-status:
	@echo "Mengecek status riwayat migrasi basis data..."
	@./scripts/migrate.sh status

# Test suites
test: test-all

test-unit:
	@echo "Running Unit Tests across workspace libraries..."
	@cargo test --workspace --lib

test-smoke:
	@echo "Running Smoke Tests (Boot, Health, Swagger UI, Infrastructure Reachability)..."
	@cargo test -p server --test smoke_test

test-integration:
	@echo "Running Integration Tests (API Routing, Request ID, CORS, OpenAPI Schemas)..."
	@cargo test -p server --test integration_test

test-performance:
	@echo "Running Performance & SLA Benchmark Tests (p95 Latency SLA, Tokio Concurrency)..."
	@cargo test -p server --test performance_test

test-reliability:
	@echo "Running Reliability, Fault Injection & Invariant Tests (Offline DB, Mockall, Zero Panics)..."
	@cargo test -p server --test reliability_test

test-all:
	@echo "Running Complete Test Suite (Unit, Smoke, Integration, Performance, Reliability)..."
	@cargo test --workspace


# Check compilation across all crates (backend & WASM frontend)
check:
	@echo "Checking backend and shared crates..."
	@cargo check --workspace
	@echo "Checking WASM frontend crate..."
	@cargo check -p web --target wasm32-unknown-unknown

# Production release build
build:
	@echo "Building production release binaries..."
	@cargo build --workspace --release
	@echo "Building production WASM frontend bundle..."
	@cd crates/web && trunk build --release

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf crates/web/dist

.PHONY: dev dev-server dev-web db-up db-down db-prune db-logs db-shell redis-shell migrate-up migrate-down migrate-status test test-unit test-smoke test-integration test-performance test-reliability test-all check build clean

