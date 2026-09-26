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

### Backlog Implementasi Kode Sumber (Fase 1 Menuju MVP)
Rincian tugas terstruktur, matriks ketergantungan kausal, dan pemetaan silang (*cross-domain matrix*) dikelola pada **[knowledge/tasks/README.md](knowledge/tasks/README.md)** serta divisualisasikan pada kanban interaktif **[obsidian-vault/pipeline.canvas](obsidian-vault/pipeline.canvas)**:

1. **[SELESAI] Fase 0 — Fondasi Monorepo & Migrasi Database:**
   - Workspace Cargo 5 crates (`domain`, `shared`, `infra`, `server`, `web`) lulus validasi kompilasi.
   - 6 pasang skrip migrasi SQL dieksekusi ke PostgreSQL 17: 14 tabel dan 37 indeks disahkan.
2. **[AKTIF / PRIORITAS 1] Task 01 — Infrastruktur, AppConfig Modular & Entitas SeaORM:**
   - Spesifikasi: [knowledge/tasks/01_infrastructure_and_config.md](knowledge/tasks/01_infrastructure_and_config.md)
   - Lingkup: Integrasi `dotenvy`, sub-configs modular, pool SeaORM dinamis, pemodelan 13 entitas SeaORM.
3. **[PRIORITAS 2] Task 02 — Sistem Otentikasi, JWT/Argon2id, OTP & Rekonsiliasi Tamu:**
   - Spesifikasi: [knowledge/tasks/02_authentication_and_user.md](knowledge/tasks/02_authentication_and_user.md)
   - Lingkup: Endpoint auth (SRS 1-7), hashing Argon2id, OTP SHA-256 via Redis, rotasi token, auto-merge (`/merge`), RBAC guard.
4. **[PRIORITAS 3] Task 03 — Backend Katalog Buku, FTS Trigram, Reader API & Gamifikasi Streak:**
   - Spesifikasi: [knowledge/tasks/03_catalog_and_reader_backend.md](knowledge/tasks/03_catalog_and_reader_backend.md)
   - Lingkup: Endpoint katalog kursor (SRS 8-12), search FTS+Trigram (<3ms), konten bab, sinkronisasi progres (SRS 13-14), heartbeat streak & lencana (SRS 15-16).
5. **[PRIORITAS 4] Task 04 — Subsistem Semantik AI, Embedding 768-Dim & Kartu Wawasan Atomik:**
   - Spesifikasi: [knowledge/tasks/04_semantic_ai_and_insights.md](knowledge/tasks/04_semantic_ai_and_insights.md)
   - Lingkup: Dual-mode embedding (Gemini & FastEmbed), Scoped Quote Finder HNSW (<10ms), cache kartu atomik bab Deepstash-style (SRS 17-20).
6. **[PRIORITAS 5] Task 05 — Ingestion Pipeline EPUB Asinkron & Admin Management:**
   - Spesifikasi: [knowledge/tasks/05_ingestion_pipeline_and_admin.md](knowledge/tasks/05_ingestion_pipeline_and_admin.md)
   - Lingkup: Upload EPUB admin (SRS 21-23), worker Redis Streams, sanitasi HTML klasik, scene chunking, monitoring.
7. **[PRIORITAS 6] Task 06 — Frontend Leptos WASM Web Reader & Mode Offline:**
   - Spesifikasi: [knowledge/tasks/06_frontend_leptos_web_reader.md](knowledge/tasks/06_frontend_leptos_web_reader.md)
   - Lingkup: Leptos WASM SPA, styling Vintage Literary (1900-1950), reader multi-column, anchor CFI, offline IndexedDB (`rexie`), i18n switcher.
8. **[PRIORITAS 7] Task 07 — Validasi Kualitas Menyeluruh, SLA Benchmarking & Rilis MVP:**
   - Spesifikasi: [knowledge/tasks/07_quality_assurance_and_launch.md](knowledge/tasks/07_quality_assurance_and_launch.md)
   - Lingkup: Suite test (smoke, unit, integration), benchmarking SLA (API <50ms, FTS <5ms, quote <10ms), audit Zero Emoji & Zero Panics, rilis MVP.

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
| ADR-12 | Eksekusi Migrasi & Skema | Eksekusi 6 migrasi SQL PostgreSQL 17, pengesahan 16 indeks matriks ERD, runner Makefile | [MEMORY.md](MEMORY.md) |
| ADR-13 | Tooling API & Pengujian | Utoipa OpenAPI/Swagger, Hot-Reload cargo-watch, Structured Logging JSON, dan 4-Tier Test Suite | [MEMORY.md](MEMORY.md) |





