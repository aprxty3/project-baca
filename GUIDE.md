---
type: Developer Guide
title: "Project Baca — Panduan Pengembang & Operasional Lokal"
description: "Panduan langkah-demi-langkah untuk penyiapan lingkungan dev, database migrasi, kompilasi WASM, dan eksekusi layanan lokal."
tags: [guide, onboarding, developer, setup, workflow, okf]
---

# GUIDE.md — Panduan Pengembang & Operasional Lokal

Panduan praktis untuk menjalankan, membangun, menguji, dan mengembangkan ekosistem **Project Baca** pada mesin lokal.

---

## 1. Prasyarat Lingkungan Pengembangan

Pastikan utilitas berikut telah terpasang pada lingkungan sistem operasi:

1. **Rust Toolchain:**
   - Versi stabil Rust 1.80+ (direkomendasikan 1.85+ atau 1.98+).
   - Target WASM: `rustup target add wasm32-unknown-unknown`.
   - Linter & Formatter: `rustup component add clippy rustfmt`.
2. **WASM Bundler:**
   - Trunk: `cargo install trunk` (atau melalui package manager OS: `pacman -S trunk`).
3. **Container Runtime:**
   - Docker & Docker Compose (v2.20+).
4. **Python Environment (untuk Worker Ingestion):**
   - Python 3.11+ dan pengelola paket `uv` atau `venv`.

---

## 2. Menjalankan Layanan Infrastruktur Lokal

Project Baca menggunakan `docker-compose.yml` untuk mengorkestrasi layanan pendukung lokal:

```bash
# Menjalankan seluruh kontainer pendukung di latar belakang
make db-up
# Atau secara manual:
docker compose up -d

# Memeriksa status kontainer
docker compose ps
```

### Pemetaan Port Layanan Lokal:
* **PostgreSQL 17 + pgvector:** `localhost:5433` (User: `baca_user`, DB: `project_baca_db`)
* **Redis 7 (Streams & Cache):** `localhost:6380`
* **MinIO Object Storage:**
  - S3 API: `localhost:9000` (Access Key: `minioadmin`, Secret: `minioadmin`)
  - Web Console: `http://localhost:9001`
* **Mailpit (SMTP Dev Server):**
  - SMTP Server: `localhost:1025`
  - Web UI: `http://localhost:8025`

---

## 3. Eksekusi Skrip Migrasi Database

Skema tabel dikelola melalui berkas SQL berpasangan di direktori `migrations/`:

```bash
# Menerapkan seluruh migrasi yang tertunda
make migrate-up

# Membatalkan (rollback) migrasi terakhir
make migrate-down

# Mereset database dari awal (rollback total lalu migrate-up)
make migrate-reset
```

---

## 4. Menjalankan Komponen Aplikasi Monorepo

### A. Perbedaan `make dev` vs `make dev-server` (Mekanisme Hot Reload)

Dalam monorepo polyglot Project Baca, backend dan frontend memiliki siklus kompilasi serta rantai alat (*toolchain*) yang independen:
* **Backend Axum:** Mengompilasi kode Rust ke biner native CPU arsitektur host (port 8080).
* **Frontend Leptos:** Mengompilasi kode Rust ke biner WebAssembly (`wasm32-unknown-unknown`) dan membundel aset web via Trunk (port 3000).

Oleh karena itu, alur eksekusi dirancang sebagai berikut:
1. **`make dev` (Orkestrator & Panduan):**
   Berfungsi sebagai navigator lingkungan dev lokal yang menampilkan status layanan dan petunjuk eksekusi. Perintah ini tidak menjalankan hot-reload langsung secara bersamaan agar aliran log kompilasi backend dan frontend tidak bertabrakan (*log interleaving*) di satu terminal.
2. **`make dev-server` (Backend Live-Reload — Ekuivalen `Air` di Golang):**
   Menjalankan server Axum dengan pemantauan otomatis via `cargo-watch`:
   ```bash
   make dev-server
   # Memantau crates/server, crates/infra, crates/domain, dan crates/shared
   # Biner akan dikompilasi ulang otomatis saat berkas disimpan
   ```
3. **`make dev-web` (Frontend Native Hot-Reload):**
   Menjalankan frontend Leptos WASM menggunakan Trunk:
   ```bash
   make dev-web
   # Trunk menyajikan live-reload otomatis via WebSocket di http://localhost:3000
   ```

### B. Dokumentasi Interaktif OpenAPI & Swagger UI (`utoipa`)

Server backend mengintegrasikan framework dokumentasi `utoipa` dan `utoipa-swagger-ui`:
* **Swagger UI:** Akses antarmuka visual pengetesan API di `http://localhost:8080/swagger-ui`.
* **OpenAPI 3.1 JSON Specification:** Unduh spesifikasi mentah di `http://localhost:8080/api-docs/openapi.json`.
* **Karakteristik Teknis:** Schema OpenAPI diperiksa secara ketat saat waktu kompilasi (*compile-time type checked*) dari DTOs pada `crates/shared`, mencegah dokumentasi kedaluwarsa.

### C. Logging Terstruktur & Korelasi Permintaan (Observability)

Sistem pencatatan log menggunakan crate `tracing` dan `tower-http`:
* **Mode Teks Human-Readable (Default Lokal):**
  Format teks berwarna untuk kenyamanan membaca log saat pengembangan lokal.
* **Mode Structured JSON (Lingkungan Produksi/Staging):**
  Aktifkan melalui variabel lingkungan:
  ```bash
  LOG_FORMAT=json cargo run -p server
  ```
* **Korelasi Request ID (`x-request-id`):**
  Setiap permintaan HTTP secara otomatis diperiksa atau diterbitkan UUIDv4 baru oleh middleware `request_id_middleware`, disematkan ke dalam span tracing, dan dikembalikan melalui header respon `x-request-id`.

### D. Background Ingestion Worker (Python)
```bash
cd python_worker
uv venv
source .venv/bin/activate
uv pip install -r requirements.txt
python -m ingestion.worker
# Worker akan terhubung ke Redis Streams di port 6380
```

---

## 5. Pengujian & Verifikasi Kualitas (4-Tier Test Suite)

Arsitektur pengujian Project Baca mengadopsi struktur komprehensif 4 lapis yang setara dengan standar pengujian backend enterprise:

```bash
# 1. Menjalankan Smoke Tests (Booting, health check, Swagger UI, keterjangkauan infrastruktur)
make test-smoke

# 2. Menjalankan Integration Tests (Alur korelasi x-request-id, CORS, kepatuhan skema OpenAPI DTO)
make test-integration

# 3. Menjalankan Performance & SLA Benchmark Tests (Validasi latensi p95 < 50ms & konkurensi 50 task)
make test-performance

# 4. Menjalankan Reliability & Fault Injection Tests (Fallback basis data offline, mockall, zero panics)
make test-reliability

# 5. Menjalankan Unit Tests (Validasi model domain murni dan validasi DTOs)
make test-unit

# 6. Menjalankan Seluruh Suite Pengujian (End-to-End)
make test-all
```

### Framework & Pustaka Pengujian yang Digunakan:
* **Mocking:** `mockall` (generasi mock otomatis via `#[automock]` di atas trait port).
* **Asersi Visual Berwarna:** `pretty_assertions` (memberikan laporan diff jelas saat asersi gagal).
* **Asersi Ekspresif:** `claims` (`assert_ok!`, `assert_err!`, `assert_matches!`).
* **Pengujian Parameterized:** `rstest` (fixture injection dan table-driven tests).


---

## 6. Pemecahan Masalah Umum (Troubleshooting)

1. **Port PostgreSQL 5433 Konflik:**
   - Periksa apakah instance Postgres lain sedang berjalan pada port tersebut: `lsof -i :5433`.
   - Ubah port mapping di `docker-compose.yml` bila diperlukan.
2. **Kompilasi WASM Gagal (`wasm32-unknown-unknown` not found):**
   - Jalankan `rustup target add wasm32-unknown-unknown`.
3. **Koneksi MinIO S3 Ditolak:**
   - Pastikan bucket `baca-epubs` dan `baca-covers` telah dibuat otomatis oleh inisialisasi Docker Compose.
4. **Redis Streams Consumer Group Tidak Ditemukan:**
   - Worker akan menginisialisasi consumer group `ingestion-group` secara otomatis dengan fallback `XGROUP CREATE`.
