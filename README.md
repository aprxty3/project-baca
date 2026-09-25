# Project Baca

Aplikasi E-Reader modern yang memadukan kenyamanan membaca mendalam ala **Kindle & Apple Books** (tipografi bersih, reflowable, bebas distraksi) dengan ringkasan wawasan atomik (*atomic insight cards*) ala **Deepstash & Blinkist**, serta kemampuan pencarian semantik kutipan (*quote finder*).

Dibangun dengan arsitektur **Polyglot Monorepo** (Rust + Python + Dual-Mode Embedding), berorientasi **100% gratis, bebas iklan, dan offline-first**, menyajikan naskah pilihan dari ranah publik (*Public Domain & Open License*).

---

## 1. Arsitektur & Tumpukan Teknologi

### A. Runtime Aplikasi Produksi
* **Controller & Web Core:** [Rust](https://www.rust-lang.org/) — [Axum](https://github.com/tokio-rs/axum) (HTTP API Gateway, Auth, WebSocket).
* **Frontend Web PWA:** [Leptos 0.7](https://leptos.dev/) (WebAssembly / WASM) — Paginasi reflowable, IndexedDB via `rexie`, dan sakelar bahasa antarmuka reaktif (`[ ID | EN ]`).
* **AI Ingestion & NLP Worker:** Python — Ekstraksi EPUB, sanitasi naskah, chunking berbasis scene/paragraf, dan integrasi Google GenAI SDK (Gemini).
* **Mesin Embedding Vektor:** Dual-Mode Provider — Google GenAI Gemini API (768-dim) untuk cloud tanpa GPU, dan FastEmbed CPU ONNX untuk lingkungan dev/offline lokal teroptimasi ARM64/x86.
* **Database & Vektor:** PostgreSQL 17 + [`pgvector`](https://github.com/pgvector/pgvector) — Indeks HNSW untuk pencarian kutipan terisolasi per buku (*scoped quote search*), serta ekstensi `pg_trgm` dan Full-Text Search untuk pencarian katalog buku.
* **Object Storage:** S3-Compatible / [Cloudflare R2](https://www.cloudflare.com/developer-platform/r2/) / MinIO — Penyimpanan file EPUB mentah dan aset sampul.
* **Cache & Task Queue:** Redis 7 — Manajemen session, rate limiting kueri semantik, dan Redis Streams untuk antrean ingestion.
* **Email Transaksional:** SMTP (Mailpit untuk dev lokal, Resend/SES untuk produksi).
* **Desain Visual:** Estetika **Vintage Literary / Mid-Century Writer (1900–1950)** dengan palet espresso `#1F1916`, tombol terakota `#CE734E`, teks perkamen `#F0EAE1`, dan ilustrasi etsa pena klasik.

### B. Infrastruktur Kecerdasan Pengembangan (Quad-Layer System One)
Memandu kolaborasi pengembang dan agen AI (**Google Antigravity** sebagai Architect/PM dan **Claude Code** sebagai Lead Engineer):
* **Layer 0 (Jev / Laya):** System-One reflex gate (<0.2 ms) untuk klasifikasi intent perintah dan guardrail keselamatan shell.
* **Layer 1 (Graphify):** Codebase AST & symbol graph untuk navigasi kode monorepo via `graphify-out/`.
* **Layer 2 (OKF Vault v0.2):** Vault dokumentasi di `knowledge/` (`prd.md`, `erd.md`, `ux-flow.md`, `frd.md`, `srs.md`) sebagai sumber kebenaran tunggal (*SSOT*).
* **Layer 3 (GBrain):** PostgreSQL 17 pgvector untuk memori keputusan lintas sesi kerja agen.

---

## 2. Navigasi Dokumentasi Lengkap

### A. Spesifikasi Inti & Kebutuhan Sistem (OKF v0.2)
* **Katalog Induk & Peta Peran Direktori:** [knowledge/index.md](knowledge/index.md)
* **Kebutuhan Produk (PRD):** [knowledge/prd.md](knowledge/prd.md)
* **Kebutuhan Fungsional & RTM (FRD):** [knowledge/frd.md](knowledge/frd.md)
* **Skema Database & 16 Matriks Indeks (ERD):** [knowledge/erd.md](knowledge/erd.md)
* **Desain Interaksi & Wireframe (UX Flow):** [knowledge/ux-flow.md](knowledge/ux-flow.md)
* **Kontrak API & Keamanan Sistem (SRS):** [knowledge/srs.md](knowledge/srs.md)
* **Audit Trail Perubahan Spesifikasi:** [knowledge/log.md](knowledge/log.md)

### B. Cetak Biru Arsitektur & Rekayasa
* **Cetak Biru Monorepo Polyglot:** [ARCHITECTURE.md](ARCHITECTURE.md)
* **Arsitektur Pemrosesan Terdistribusi & Worker:** [DISTRIBUTED.md](DISTRIBUTED.md)
* **Log Keputusan Arsitektural (ADR):** [MEMORY.md](MEMORY.md)
* **Katalog Aset & Ilustrasi Vintage:** [assets/README.md](assets/README.md)
* **Pernyataan Legalitas & Lisensi Domain Publik:** [NOTICE.md](NOTICE.md)
* **Riwayat Rilis & Milestone:** [CHANGELOG.md](CHANGELOG.md)
* **Status Sprint & Backlog Aktif:** [PROJECT_LOG.md](PROJECT_LOG.md)

### C. Protokol Kolaborasi Multi-Agen AI
* **Panduan Umum Seluruh AI Coding Agent:** [AGENTS.md](AGENTS.md)
* **Panduan Pengembang Claude Code:** [CLAUDE.md](CLAUDE.md)
* **Panduan Google Antigravity & Model Gemini:** [GEMINI.md](GEMINI.md)

---

## 3. Memulai Lingkungan Pengembangan Lokal

Panduan langkah-demi-langkah, konfigurasi prasyarat, dan pemecahan masalah tersedia lengkap pada **[GUIDE.md](GUIDE.md)**.

Ringkasan cepat menjalankan layanan:

```bash
# 1. Jalankan layanan infrastruktur (PostgreSQL 17, Redis 7, MinIO, Mailpit)
make db-up
# Atau: docker compose up -d

# 2. Jalankan backend REST API (Axum)
cargo run --bin project-baca-server

# 3. Jalankan frontend Web Reader (Leptos WASM via Trunk)
cd crates/web && trunk serve --port 3000
```

