---
type: Memory
title: "Project Baca — Log Keputusan Arsitektur & Memori Sistem"
description: "Log kronologis keputusan teknis dan arsitektural beserta alasannya. Sumber kebenaran untuk 'kenapa'."
tags: [memory, decisions, architecture, history, context, okf]
---

# MEMORY.md — Log Keputusan Arsitektur & Memori Sistem

Dokumen ini mencatat rasionalisasi mendalam (*the "why"*) di balik setiap keputusan teknis dan arsitektur pada **Project Baca**. Siapa pun (pengembang manusia maupun agen AI) yang bergabung ke proyek ini dapat memahami alasan fundamental sebuah keputusan diambil tanpa perlu merekonstruksi riwayat percakapan dari awal.

Setiap entri mencantumkan identitas pelaku (*Actor*): `[antigravity]`, `[claude]`, atau `human:aprxty3`.

---

## Indeks Tematik Keputusan

* **Arsitektur Inti & Tumpukan Bahasa:** Monorepo Polyglot (Rust Axum + Leptos WASM + Python Worker + Dual-Mode Embedding).
* **Mesin Embedding Vektor:** Eliminasi Triton Server; Adopsi Gemini API (Cloud) & FastEmbed CPU (ARM64/x86); Standardisasi Vektor 768-Dimensi.
* **Basis Data & Mesin Pencarian:** "Postgres for Everything" (FTS + pg_trgm + pgvector HNSW); penolakan Elasticsearch.
* **Message Broker & Task Queue:** Penggunaan Redis Streams; penolakan RedPanda/RabbitMQ untuk fase MVP.
* **Lapisan Data & Migrasi:** Adopsi SeaORM; migrasi SQL berpasangan (`.up.sql` dan `.down.sql`).
* **Penyimpanan Objek & Email:** S3-compatible (MinIO / Cloudflare R2); Mailpit untuk SMTP dev lokal.
* **Desain Pengalaman Pengguna:** Estetika *Vintage Literary (1900–1950)*; isolasi UI i18n dwibahasa `[ID|EN]`.
* **Disiplin Rekayasa Perangkat Lunak:** Kodifikasi 6 Pilar Rekayasa (ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, YAGNI) dan Kebijakan Bebas Emoji.
* **Tata Kelola Pengetahuan:** Implementasi Open Knowledge Format (OKF v0.2), Graphify Knowledge Graph, dan Obsidian Vault.

---

## Log Keputusan Kronologis

### 2026-09-26 — Adopsi Pemetaan Graf 3-Dimensi & Kanban Canvas Siklus Hidup Monorepo
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Roadmap pengerjaan Task 01–07 membutuhkan keterhubungan yang tidak hanya linear, melainkan mampu menavigasikan pengembang dan agen AI melintasi 4 pilar monorepo (Spesifikasi OKF, Crate Rust, Basis Data/Indeks, dan Alur Pengguna/SLA).
* **Keputusan:**
  1. **Directed Dependency Wikilinks:** Menyisipkan metadata `Prasyarat Kausal`, `Membuka Tahap`, dan `Jalur Kritis` di setiap berkas task agar Graphify menghasilkan relasi `depends_on` yang terstruktur.
  2. **Matriks Silang Antar-Lapis (Cross-Domain Matrix):** Memetakan relasi silang vertikal di setiap dokumen task menuju berkas kode sumber Rust (`crates/*`), skema SQL (`migrations/*`), dan dokumen spesifikasi (`knowledge/*`).
  3. **Visualisasi Alur Kerja via Obsidian Canvas:** Menerbitkan `obsidian-vault/pipeline.canvas` sebagai kanban interaktif alur kerja menuju peluncuran MVP.
  4. **Federasi Vektor GBrain:** Mendaftarkan sumber `baca-knowledge` pada GBrain dengan konfigurasi `--include-gitignored` dan sinkronisasi berkala via script `sync-knowledge`.

---

### 2026-09-25 — Eliminasi Triton Inference Server & Adopsi Dual-Mode Embedding (768 Dimensi)
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Mesin pengembangan Linux lokal tidak memiliki dedicated GPU (tanpa NVIDIA CUDA / AMD ROCm), dan server produksi menggunakan CPU arsitektur ARM (misal Ampere Altra / AWS Graviton). Menjalankan kontainer Triton Inference Server pada CPU ARM menimbulkan pemborosan Docker image (10–15 GB), konsumsi RAM idle 1–2 GB, tanpa mendapatkan manfaat akselerasi GPU sama sekali.
* **Keputusan:**
  1. **Mengeliminasi Triton Inference Server:** Dihapus sepenuhnya dari tumpukan MVP demi kepatuhan ketat terhadap prinsip **KISS** (Keep It Simple, Stupid) dan **YAGNI** (You Aren't Gonna Need It).
  2. **Adopsi Arsitektur Dual-Mode Embedding Provider:**
     - **Mode Cloud (Default Produksi):** Google GenAI Gemini API (`text-embedding-004`). Memproses vektor di cloud tanpa GPU dan tanpa membebani memori RAM lokal.
     - **Mode Lokal / Offline:** Pustaka `fastembed` berbasis ONNX Runtime yang dioptimalkan khusus untuk CPU (termasuk akselerasi ARM NEON). Ukuran model hanya ~60 MB dan memori RAM <150 MB.
  3. **Standardisasi Vektor 768 Dimensi:** Mengubah dimensi vektor embedding dari 1536 menjadi **768 dimensi**, memangkas alokasi memori RAM indeks HNSW PostgreSQL `pgvector` hingga 50%.

---

### 2026-09-25 — Implementasi Lengkap OKF v0.2 & Integrasi Obsidian Vault
* **Aktor:** `[antigravity]`, ditinjau oleh `human:aprxty3`
* **Konteks:** Diperlukan standarisasi tata kelola pengetahuan (*Knowledge Management*) dan aturan agen persisten agar seluruh agen AI patuh terhadap batasan monorepo.
* **Keputusan:**
  1. Menstandarisasi [knowledge/index.md](knowledge/index.md) dengan *Directory Role & Responsibility Map* dan hierarki 4 lapis *Progressive Disclosure*.
  2. Menerbitkan aturan agen persisten di [.agents/rules/okf_memory.md](.agents/rules/okf_memory.md), [.agents/rules/graphify.md](.agents/rules/graphify.md), dan workflow [.agents/workflows/graphify.md](.agents/workflows/graphify.md).
  3. Mengekstrak graf pengetahuan via Graphify (Gemini AST) menghasilkan 20 nodes, 15 komunitas di `graphify-out/`, dan mengekspor vault terhubung ke `/home/aprxty3/ObsidianVaults/project-baca/` dengan symlink `obsidian-vault/`.

---

### 2026-09-25 — Kodifikasi 6 Pilar Rekayasa Baku & Kebijakan Bebas Emoji
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Repositori membutuhkan pagar pembatas arsitektural (*guardrails*) untuk mencegah degradasi kualitas kode, over-engineering, atau gaya penulisan informal.
* **Keputusan:**
  1. Mengunci 6 prinsip rekayasa baku:
     - **ROBUST:** Zero panics di Rust (`unwrap()` dilarang di jalur produksi), handling kegagalan I/O terisolasi.
     - **SCALABLE:** Stateless Axum API, scoped semantic search terisolasi per buku (`WHERE book_id = $1`).
     - **EASY TO MAINTAIN:** Crate modular yang terpisah jelas, migrasi skema SQL terkelola, SSOT OKF v0.2.
     - **DRY:** Shared DTOs dan kamus validasi terpusat di `crates/shared`.
     - **KISS:** Arsitektur database tunggal (Postgres), antrean tunggal (Redis Streams).
     - **YAGNI:** Fokus ketat pada MVP web reader dan pencarian semantik; menunda fitur spekulatif.
  2. Memberlakukan **Zero Emoji Policy**: Melarang seluruh emoji, emoticon grafis, dan gaya bahasa santai di dalam dokumen teknis, kode, komentar, dan commit message.

---

### 2026-09-25 — Keputusan Database: "Postgres for Everything" vs Elasticsearch
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Evaluasi kebutuhan fitur pencarian buku: apakah memerlukan dedicated search engine seperti Elasticsearch untuk pencarian katalog awal?
* **Keputusan:**
  - **Menolak Elasticsearch:** Elasticsearch membutuhkan alokasi RAM JVM 2–4 GB, menambah overhead sinkronisasi ganda (*dual-write*), dan memperbesar resiko split-brain.
  - **Memilih PostgreSQL 17 FTS + `pg_trgm` GIN Index:** Mampu melayani pencarian leksikal dengan latensi <3ms dan toleransi saltik (*typo tolerance*) tanpa infrastruktur tambahan.
  - **Pencarian Semantik via `pgvector` HNSW Index:** Melayani *scoped quote search* dengan latensi <10ms langsung di PostgreSQL 17.

---

### 2026-09-25 — Evaluasi Message Broker: Redis Streams vs RedPanda / RabbitMQ
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Kebutuhan antrean asinkron untuk proses ingestion file EPUB dan pengiriman email verifikasi OTP.
* **Keputusan:**
  - Menggunakan **Redis Streams** (yang sudah tersedia di tumpukan Redis 7 pendukung sesi dan rate-limiting).
  - Menolak dedicated broker seperti RedPanda atau RabbitMQ karena over-engineering untuk skala MVP (pelanggaran prinsip KISS dan YAGNI).

---

### 2026-09-25 — Manajemen Data: SeaORM & Migrasi SQL Berpasangan
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Memastikan integritas manipulasi data di backend Rust dan riwayat evolusi skema tabel yang dapat diaudit dan di-rollback.
* **Keputusan:**
  - Mengadopsi **SeaORM** untuk kueri asinkron *type-safe* dan pemetaan relasi entitas domain.
  - Mengelola skema melalui direktori `migrations/` dengan berkas SQL berpasangan (`<timestamp>_<nama>.up.sql` dan `<timestamp>_<nama>.down.sql`).
  - Mengorkestrasi seluruh siklus database melalui `Makefile` (`make migrate-up`, `make migrate-down`, `make migrate-reset`).

---

### 2026-09-25 — Desain Visual & Pengalaman Pengguna: Vintage Literary (1900–1950)
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Menentukan diferensiasi visual platform e-reader buku naskah klasik ranah publik.
* **Keputusan:**
  - Menghindari antarmuka SaaS modern datar (*flat modern UI*).
  - Mengadopsi estetika cetak klasik zaman keemasan sastra: palet aged paper `#F9F6F0`, tinta cetak iron gall `#2B2625`, aksen terakota `#9D5A3C`, tipografi serif sastrawan klasik ala `gbrain.io` paper theme, dan pembatas fleuron klasik `❖`.
  - Mengintegrasikan ringkasan bergaya **Deepstash & Blinkist** (*Atomic Insight Cards*) dan *Catch-up Recap* untuk mempermudah pemahaman naskah sastra klasik yang padat.

---

### 2026-09-25 — Ekspansi Graf Pengetahuan Graphify & Sinkronisasi Vault Obsidian
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Penambahan berkas tata kelola multi-agen root (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `MEMORY.md`, dll.) serta pengabaian git untuk `knowledge/` menuntut pembaruan topologi graf pengetahuan tanpa kehilangan pengindeksan spesifikasi privat.
* **Keputusan:**
  1. Menerbitkan berkas `.graphifyignore` dengan negasi `!knowledge/` dan `!knowledge/**` untuk menjembatani kebutuhan privasi git (spesifikasi di-ignore dari push) dengan kebutuhan graf lokal (spesifikasi tetap diindeks sebagai SSOT).
  2. Melakukan ekstraksi inkremental via `graphify extract . --backend gemini`, memperluas graf dari 20 simpul menjadi 51 simpul dan 32 komunitas terklaster.
  3. Menyinkronkan kembali vault Obsidian lokal di `obsidian-vault/` dengan 83 catatan terhubung, kanvas visual `graph.canvas`, dan dasbor navigasi terpadu `00_DASHBOARD.md`.

---

### 2026-09-26 — Inisialisasi Monorepo Cargo Workspace & Skeleton Multi-Crate
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Eksekusi Fase 1 implementasi kode sumber Project Baca membutuhkan fondasi monorepo yang modular, mematuhi prinsip DRY (berbagi DTO antara backend dan WASM), serta menjamin Zero Panics di jalur produksi.
* **Keputusan:**
  1. **Workspace Resolver v2:** Mengatur `Cargo.toml` root dengan dependensi bersama terpusat (`[workspace.dependencies]`), menghindari duplikasi versi pustaka dan menjamin keselarasan compiler.
  2. **Pemisahan Lima Crate Fungsional:**
     - `crates/shared`: Komponen bebas platform (DTO, API Response, AppError) yang dapat dikompilasi ke target native x86/ARM maupun `wasm32-unknown-unknown`.
     - `crates/domain`: Model entitas murni yang terisolasi dari basis data dan framework HTTP.
     - `crates/infra`: Enkapsulasi koneksi database SeaORM (PostgreSQL 17) dan Redis client, dengan fallback graceful saat dev offline.
     - `crates/server`: HTTP API berbasis Axum 0.8 dengan routing terstruktur, CORS, tracing, dan graceful shutdown listener.
     - `crates/web`: Single Page Application berbasis Leptos 0.7.8 WASM dan Trunk, mengimplementasikan estetika *Vintage Literary (1900–1950)* dan reaktif language switcher.
  3. **Otomasi Terpusat Makefile:** Memastikan seluruh alur build, check, dev, test, dan manajemen container database dapat dijalankan dengan satu perintah konsisten.

---

### 2026-09-26 — Eksekusi Migrasi Basis Data PostgreSQL 17 & Pengesahan Skema 16 Indeks
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Menindaklanjuti spesifikasi ERD (`knowledge/erd.md`), diperlukan realisasi skema basis data fisik pada PostgreSQL 17 lokal beserta pengujian 16 indeks komprehensif untuk pencarian leksikal dan semantik.
* **Keputusan:**
  1. **Migrasi SQL Berpasangan Terkelola (`migrations/`):**
     - Menerbitkan 6 pasang berkas migrasi berstempel waktu yang mencakup seluruh 13 tabel domain dan relasinya.
     - Menyusun runner migrasi modular `scripts/migrate.sh` dengan tabel pelacak `schema_migrations` agar eksekusi migrasi bersifat deterministik, terurut, dan reversibel via `make migrate-up` dan `make migrate-down`.
  2. **Pengesahan 16 Indeks Matriks ERD:**
     - Mengesahkan indeks parsial GIN FTS (`idx_books_published_fts`) dan Trigram (`idx_books_published_title_trgm`, `idx_books_published_author_trgm`) dengan filter `WHERE status = 'published'` guna memangkas konsumsi RAM hingga 80%.
     - Mengesahkan indeks HNSW pgvector 768 dimensi (`idx_book_chunks_hnsw_embedding`) dengan parameter `vector_cosine_ops WITH (m = 16, ef_construction = 64)` untuk latensi pencarian kutipan di bawah 10 milidetik.
  3. **Penyesuaian Infrastruktur MinIO:**
     - Mengalihkan image MinIO ke `cgr.dev/chainguard/minio:latest` karena image Docker Hub telah diarsipkan oleh penyedia hulu.
     - Mengalihkan port host MinIO ke `9005:9000` dan console ke `9006:9001` untuk mencegah konflik dengan proses lokal lain yang menggunakan port 9000.

---

### 2026-09-26 — Kodifikasi Task Breakdown Terstruktur Menuju MVP (`knowledge/tasks/`)
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Menghadapi implementasi menyeluruh dari monorepo polyglot, diperlukan peta jalan pelaksanaan tugas yang berurutan, terukur, dan terhubung langsung ke spesifikasi SSOT pada direktori `knowledge/` agar agen AI maupun pengembang tidak melewatkan prasyarat teknis, invarian arsitektur, atau kontrak API.
* **Keputusan:**
  1. **Pembentukan Direktori `knowledge/tasks/`:**
     - Mengodifikasikan seluruh kebutuhan produk, modul fungsional, dan spesifikasi teknis menjadi 7 paket tugas bertahap (Task 01 s/d Task 07) dengan indeks induk `knowledge/tasks/README.md`.
  2. **Matriks Ketergantungan Tegas:**
     - Mengunci alur kerja berurutan: Fondasi Infrastruktur/SeaORM -> Otentikasi & Rekonsiliasi Tamu -> Reader & Katalog Backend -> Semantik AI Embedding 768-Dim -> Ingestion Pipeline Asinkron -> Frontend Leptos WASM -> QA & Hardening.
  3. **Penyelarasan Backlog & Navigasi Pengetahuan:**
     - Menghubungkan setiap tugas ke nomor User Story pada `prd.md`, modul pada `frd.md`, dan nomor endpoint pada `srs.md`.
     - Memutakhirkan `knowledge/index.md` dan `PROJECT_LOG.md` sebagai kendali operasional tunggal.

---

### 2026-09-26 — Adopsi OpenAPI Utoipa, Standarisasi Hot Reload, Logging Terstruktur & Arsitektur 4-Tier Test Suite
* **Aktor:** `human:aprxty3` & `[antigravity]`
* **Konteks:** Menjawab kebutuhan dokumentasi interaktif API setara Swaggo di Go, kejelasan eksekusi live-reload, kesiapan observabilitas produksi, serta perlunya framework pengujian komprehensif (smoke, integration, performance, reliability) yang setara dengan pola `test-backend-mkp/test/`.
* **Keputusan:**
  1. **Adopsi `utoipa` dan `utoipa-swagger-ui` (OpenAPI 3.1):**
     - Memilih `utoipa` daripada pendekatan berbasis komentar teks karena *compile-time type safety*: skema OpenAPI otomatis disinkronkan langsung dari DTOs Rust (`#[derive(ToSchema)]`).
     - Menyajikan Swagger UI interaktif di `/swagger-ui` dan spesifikasi mentah di `/api-docs/openapi.json`.
     - Mengisolasi dependensi `utoipa` pada `crates/shared` di balik fitur opsional `openapi` agar tidak membebani kompilasi WASM frontend `crates/web`.
  2. **Standarisasi Hot Reload (`cargo-watch` & `trunk serve`):**
     - Menjelaskan disparitas antara `make dev` (panduan orkestrasi lingkungan) dengan `make dev-server` (live-reload backend via `cargo-watch`) dan `make dev-web` (live-reload frontend WASM via WebSocket Trunk).
     - Menjadikan `cargo-watch` standar dev-server yang memantau seluruh 4 crate backend (`server`, `infra`, `domain`, `shared`).
  3. **Logging Terstruktur & Korelasi Permintaan:**
     - Mengonfigurasi `tracing-subscriber` dengan dukungan format JSON per baris via sakelar `LOG_FORMAT=json` untuk lingkungan produksi.
     - Mengimplementasikan `request_id_middleware` yang memetakan atau menerbitkan UUIDv4 baru ke dalam header `x-request-id` dan tracing span `http_request`.
  4. **Penerapan Arsitektur Pengujian 4 Lapis (4-Tier Test Suite):**
     - Mengintegrasikan pustaka testing standar industri: `mockall` (mocking trait port setara Mockery), `claims` & `pretty_assertions` (asersi ekspresif & diff berwarna setara Testify), serta `rstest` (parameterized testing).
     - Membangun 4 suite pengujian terpisah pada `crates/server/tests/`:
       - `smoke_test.rs`: Validasi boot, `/health`, Swagger UI, dan reachability TCP port Postgres/Redis (5 passed).
       - `integration_test.rs`: Validasi alur request-response, korelasi request ID kustom/otomatis, CORS preflight, dan integritas skema DTO OpenAPI (5 passed).
       - `performance_test.rs`: Pengukuran distribusi latensi SLA p95 < 50ms dan stabilitas 50 worker konkuren Tokio (2 passed).
       - `reliability_test.rs`: Validasi isolasi kesalahan database terputus tanpa crash (*zero panics*), fault injection via `mockall`, dan resistensi buffer header malformed (3 passed).
     - Seluruh 15 skenario pengujian diverifikasi lulus 100% via `make test-all`.




