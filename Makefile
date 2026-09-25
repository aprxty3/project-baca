.PHONY: help check build test dev-server dev-web db-up db-down db-logs clean

help: ## Tampilkan daftar perintah yang tersedia
	@echo "Project Baca — Otomasi Task Runner"
	@echo "=================================="
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-16s\033[0m %s\n", $$1, $$2}'

check: ## Periksa kompilasi seluruh workspace (backend & frontend)
	cargo check --workspace

build: ## Bangun seluruh workspace binary (release mode)
	cargo build --workspace --release

test: ## Jalankan seluruh unit & integration test
	cargo test --workspace

dev-server: ## Jalankan backend server Axum
	cargo run -p server

dev-web: ## Jalankan frontend Leptos WASM dev server via Trunk
	cd crates/web && trunk serve

db-up: ## Jalankan kontainer database lokal (PostgreSQL 17, Redis 7, MinIO, Mailpit)
	docker compose up -d

db-down: ## Hentikan seluruh kontainer database lokal
	docker compose down

db-logs: ## Pantau log kontainer database
	docker compose logs -f

clean: ## Bersihkan artefak build Cargo dan Trunk
	cargo clean
	rm -rf crates/web/dist
