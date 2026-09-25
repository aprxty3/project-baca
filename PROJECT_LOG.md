# Log & Memory Proyek: Project Baca

File ini mencatat status terkini, keputusan teknis, dan rencana kerja (*backlog*) untuk **Project Baca**.

---

## 1. Status Terkini (State: Inisialisasi & Perencanaan Arsitektur)
* **Tanggal:** 2026-09-25
* **Target:** E-Reader Apps (Web PWA Leptos WASM, Mobile via Tauri v2, RAG/Semantic Subsystem).
* **Ekosistem Lokal & Infrastruktur:**
  * Toolchain Rust: `rustup` default stable (1.98.1), target `wasm32-unknown-unknown` aktif.
  * WASM Bundler: `trunk` terpasang via pacman.
  * Docker Compose (`docker-compose.yml`):
    * PostgreSQL 17 + `pgvector` di port `5433`.
    * Redis 7 di port `6380` (Session, Rate Limiting & Redis Streams queue).
    * MinIO (S3-compatible) di port `9000` (API) & `9001` (Console).
    * Mailpit (SMTP server dev) di port `1025` (SMTP) & `8025` (Web UI).
  * Spesifikasi yang Sudah Terbit:
    * [ARCHITECTURE.md](ARCHITECTURE.md) (Cetak Biru Monorepo Polyglot, Ingestion Pipeline, Scoped Quote Search).
    * [README.md](README.md) (Ikhtisar tumpukan teknologi & panduan dev lokal).
    * [knowledge/prd.md](knowledge/prd.md) (PRD v0.2: US-01 s/d US-15, lisensi public domain, i18n switcher, vintage visual).
    * [assets/README.md](assets/README.md) (Katalog 5 ilustrasi etsa pena klasik Victoria/Edwardian & referensi desain gbrain.io).

---

## 2. Agenda Selanjutnya (Phase Transition: Menuju Implementasi Monorepo)

Seluruh 5 dokumen spesifikasi dan arsitektur pada direktori `knowledge/` telah selesai 100%:
1. **[SELESAI] PRD (Product Requirements Document):**
   - Dokumen di [knowledge/prd.md](knowledge/prd.md) merinci visi produk, user stories MVP (US-01 s/d US-15, US-01B), model konten ranah publik, taksonomi buku, dan 6 prinsip rekayasa baku.
2. **[SELESAI] ERD (Entity Relationship Diagram):**
   - Dokumen di [knowledge/erd.md](knowledge/erd.md) mendefinisikan 13 entitas tabel PostgreSQL 17 lengkap dengan matriks 16 indeks terpadu (GIN FTS/Trigram, partial index, HNSW vector), kamus data detail, dan strategi migrasi berpasangan.
3. **[SELESAI] UX / UI Design Flow:**
   - Dokumen di [knowledge/ux-flow.md](knowledge/ux-flow.md) memetakan diagram alur navigasi lengkap, 5 wireframe detail ASCII, siklus rekonsiliasi data tamu ke akun terdaftar (`IndexedDB` -> Cloud), serta pemetaan hierarki komponen reaktif Leptos WASM.
4. **[SELESAI] FRD (Functional Requirements Document):**
   - Dokumen di [knowledge/frd.md](knowledge/frd.md) merinci spesifikasi 7 modul fungsional lengkap dengan diagram alur I/O dan Matriks Keterlacakan Kebutuhan (RTM).
5. **[SELESAI] SRS (Software Requirements Specification):**
   - Dokumen di [knowledge/srs.md](knowledge/srs.md) mendefinisikan arsitektur keamanan multi-lapis (Cloudflare edge WAF/DDoS, Redis rate-limiting, CSRF, Argon2id, OTP 6-digit, JWT refresh token rotasi), 23 kontrak endpoint REST API detail, SLA performa (API <50ms, FTS <5ms, quote <10ms), dan NFR.

### Backlog Implementasi Kode Sumber (Phase 1 Implementation)
1. **[SELESAI] Setup Fondasi Monorepo Cargo Workspace & Makefile:**
   - Inisialisasi workspace `Cargo.toml`, 5 skeleton crates (`domain`, `shared`, `infra`, `server`, `web`), bundling Trunk WASM Leptos 0.7, dan berkas `Makefile`. Seluruh `cargo check` dan `cargo test` lulus 100%.
2. **[AKTIF / PRIORITAS 1] Pembuatan Skrip Migrasi SQL (`migrations/`):**
   - Menulis 6 pasang skrip SQL `.up.sql` dan `.down.sql` berdasarkan DDL pada `knowledge/erd.md`.
3. **[PRIORITAS 2] Backend Server Core (Axum + SeaORM):**
   - Konfigurasi koneksi pool PostgreSQL 17, Redis client, middleware otentikasi JWT, rate limiting, dan endpoint auth/katalog dasar.
4. **[PRIORITAS 3] Frontend Web PWA (Leptos 0.7 WASM + Trunk):**
   - Setup styling CSS Vintage Literary (palet espresso, kertas antik, terakota), reaktif i18n switcher, dan paginasi reflowable.
5. **[PRIORITAS 4] Python Worker & Dual-Mode Embedding Pipeline:**
   - Setup parsing EPUB dan ekstraksi embedding 768-dim (Gemini API / FastEmbed CPU).

---

## 3. Rujukan Keputusan Arsitektural (SSOT: MEMORY.md)

Seluruh riwayat, rasionalisasi teknis (*the "why"*), dan atribusi aktor untuk setiap keputusan arsitektural didokumentasikan secara terpusat di **[MEMORY.md](MEMORY.md)**:

| ID | Domain Keputusan | Inti Keputusan Teknis | Rujukan Otoritatif |
|---|---|---|---|
| ADR-01 | Arsitektur Sistem | Monorepo Polyglot (Rust Axum + Leptos WASM + Python Worker + Dual-Mode Embedding) | [MEMORY.md](MEMORY.md#log-keputusan-kronologis) |
| ADR-02 | Basis Data & Pencarian | "Postgres for Everything" (FTS + Trigram + pgvector HNSW); menolak Elasticsearch | [MEMORY.md](MEMORY.md) |
| ADR-03 | Message Broker | Redis Streams tunggal untuk ingestion & email; menolak RedPanda/RabbitMQ di MVP | [MEMORY.md](MEMORY.md) |
| ADR-04 | Lapisan Data & Migrasi | SeaORM async type-safe + skrip migrasi berpasangan `.up.sql`/`.down.sql` via Makefile | [MEMORY.md](MEMORY.md) |
| ADR-05 | Identitas Desain UI | Estetika Vintage Literary (1900–1950) + Deepstash/Blinkist atomic cards | [MEMORY.md](MEMORY.md) |
| ADR-06 | Disiplin Rekayasa | Penerapan ketat 6 Pilar (ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, YAGNI) & Zero Emoji | [MEMORY.md](MEMORY.md) |
| ADR-07 | Tata Kelola Pengetahuan | OKF v0.2 Knowledge Vault, Graphify AST extraction, dan Obsidian Vault | [MEMORY.md](MEMORY.md) |
| ADR-08 | Tata Kelola Multi-Agen | Penerbitan AGENTS.md, CLAUDE.md, GEMINI.md, GUIDE.md, DISTRIBUTED.md, CHANGELOG.md, NOTICE.md | [MEMORY.md](MEMORY.md) |
| ADR-09 | Mesin Vektor & AI | Eliminasi Triton Server; Adopsi Gemini API & FastEmbed CPU (768 Dimensi) | [MEMORY.md](MEMORY.md) |
| ADR-10 | Ekspansi Graf & Vault | Pemutakhiran graf 51 simpul, .graphifyignore overrides, dan sinkronisasi Obsidian Vault | [MEMORY.md](MEMORY.md) |
| ADR-11 | Fondasi Monorepo | Monorepo 5 Crate Rust (shared, domain, infra, server, web) + Makefile & Trunk WASM | [MEMORY.md](MEMORY.md) |



