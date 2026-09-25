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

### A. Backend HTTP API (Axum)
```bash
cargo run --bin project-baca-server
# Layanan REST API akan aktif di http://localhost:8080
```

### B. Frontend Web Reader (Leptos 0.7 WASM via Trunk)
```bash
cd crates/web
trunk serve --port 3000 --open
# Aplikasi web PWA akan aktif di http://localhost:3000
```

### C. Background Ingestion Worker (Python)
```bash
cd python_worker
uv venv
source .venv/bin/activate
uv pip install -r requirements.txt
python -m ingestion.worker
# Worker akan terhubung ke Redis Streams di port 6380
```

### D. Menjalankan Seluruh Layanan Secara Bersamaan
```bash
make dev
```

---

## 5. Pengujian & Verifikasi Kualitas

Setiap perubahan kode wajib divalidasi sebelum commit:

```bash
# 1. Pemeriksaan format dan linting
make check
# Meliputi: cargo fmt --check, cargo clippy --all-targets -- -D warnings

# 2. Menjalankan suite pengujian unit dan integrasi
make test

# 3. Verifikasi performa dan SLA:
# - API general: latensi p95 < 50ms
# - Pencarian leksikal katalog FTS: < 5ms
# - Pencarian kutipan semantik pgvector: < 10ms
```

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
