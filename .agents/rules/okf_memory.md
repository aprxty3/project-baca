---
trigger: always_on
description: Open Knowledge Format (OKF v0.2) & Persistent Memory Guidelines for Project Baca.
---

## Open Knowledge Format (OKF) & Persistent Agent Rules

Workspace ini mengikuti spesifikasi **Google Cloud Open Knowledge Format (OKF v0.2)** dan navigasi graf pengetahuan Graphify sebagai Sumber Kebenaran Tunggal (*Single Source of Truth - SSOT*) arsitektur sistem.

### 1. OKF Progressive Disclosure
- Sebelum mengeksplorasi kode mentah atau menjawab pertanyaan arsitektur yang kompleks, selalu rujuk [knowledge/index.md](knowledge/index.md) untuk memetakan dokumen konsep yang berwenang.
- Gunakan alat Graphify (`graphify query "<pertanyaan>"`, `query_graph`, `shortest_path`, atau `get_node`) untuk memeriksa keterhubungan struktural antarmodul, spesifikasi, dan aset.
- Ikuti alur pengungkapan bertahap (*Progressive Disclosure*):
  - **Lapis 1 (Ringkasan & Orientasi):** [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md), dan [knowledge/index.md](knowledge/index.md).
  - **Lapis 2 (Kebutuhan Produk & Navigasi):** [knowledge/prd.md](knowledge/prd.md) dan [knowledge/ux-flow.md](knowledge/ux-flow.md).
  - **Lapis 3 (Kebutuhan Fungsional & Teknis):** [knowledge/frd.md](knowledge/frd.md), [knowledge/erd.md](knowledge/erd.md), dan [knowledge/srs.md](knowledge/srs.md).
  - **Lapis 4 (Audit Trail & Log Memori):** [knowledge/log.md](knowledge/log.md) dan [PROJECT_LOG.md](PROJECT_LOG.md).

### 2. Pencatatan Perubahan Kode Otomatis (Audit Trail)
- Setiap kali menyelesaikan perubahan kode yang bermakna, refaktor, perbaikan bug, atau pembaruan skema database:
  1. Catat pembaruan di [knowledge/log.md](knowledge/log.md) dengan mencantumkan aktor, tanggal, aksi spesifik, dan konsep/dokumen yang terdampak (*affected concepts*).
  2. Mutakhirkan [PROJECT_LOG.md](PROJECT_LOG.md) untuk status operasional backlog.
  3. Jalankan `graphify update .` atau `graphify cluster-only` untuk menjaga sinkronisasi graf pengetahuan.

### 3. Protokol Riset Mendalam & Verifikasi Faktual
- Ketika membahas pustaka baru, API crate Rust, parameter Triton, algoritma pgvector, atau keputusan desain arsitektur:
  - DILARANG berspekulasi atau menebak parameter tanpa verifikasi.
  - Lakukan riset aktif menggunakan alat pencarian atau baca dokumentasi resmi.
  - Dasarkan setiap keputusan teknis pada rujukan primer yang terverifikasi.

### 4. Core Invariants Project Baca (Non-Negotiable)
- **Zero Panics di Rust:** Penggunaan `unwrap()` atau `expect()` tanpa justifikasi dilarang pada jalur produksi. Seluruh kegagalan I/O dan jaringan wajib ditangani menggunakan `Result<T, AppError>`.
- **Postgres for Everything:** Pencarian leksikal katalog dilayani oleh PostgreSQL 17 FTS + `pg_trgm` GIN index (<3ms). Pencarian kutipan semantik dilayani oleh `pgvector` HNSW index (<10ms). Elasticsearch dilarang keras untuk menghindari pemborosan RAM dan dual-write.
- **Pragmatisme Broker:** Antrean ingestion EPUB dan pengiriman email transaksional dilayani oleh Redis Streams. Broker terpisah seperti RedPanda atau RabbitMQ dilarang untuk fase MVP demi kepatuhan prinsip KISS dan YAGNI.
- **Integritas Migrasi Database:** Seluruh perubahan skema tabel wajib dikelola melalui berkas SQL berpasangan di direktori `migrations/` (`<timestamp>_<nama>.up.sql` dan `<timestamp>_<nama>.down.sql`).
- **Otomasi Terpusat:** Seluruh perintah build, run, test, dan migrasi wajib dapat diakses melalui `Makefile`.
- **Estetika Vintage Literary (1900-1950):** Antarmuka web Leptos WASM wajib merefleksikan keanggunan era cetak klasik (palet aged paper, tinta iron gall, tipografi serif sastrawan) tanpa elemen visual digital modern yang mencolok.

### 5. Prinsip Rekayasa Perangkat Lunak Baku (Mandatory)
- **ROBUST:** Menangani seluruh kemungkinan kegagalan (koneksi database putus, rate limit tercapai, token kedaluwarsa, file EPUB korup) secara elegan tanpa crash.
- **SCALABLE:** Backend Axum stateless, pencarian kutipan semantik terisolasi per buku (`WHERE book_id = $1`), dan partial indexing untuk menghemat RAM.
- **EASY TO MAINTAIN:** Pemisahan tanggung jawab yang tegas antar-crate (`domain`, `shared`, `infra`, `server`, `web`), dokumentasi terstruktur OKF v0.2, dan struktur kode modular.
- **DRY (Don't Repeat Yourself):** DTOs dan logika validasi bersama diletakkan di `crates/shared` agar dapat digunakan oleh backend Axum maupun frontend Leptos WASM.
- **KISS (Keep It Simple, Stupid):** Pilih solusi paling sederhana yang menyelesaikan masalah secara andal tanpa layer abstraksi artifisial yang berlebihan.
- **YAGNI (You Aren't Gonna Need It):** Implementasikan hanya fitur yang telah dispesifikasikan dalam lingkup MVP saat ini. Fitur masa depan (seperti aplikasi mobile native, distributed worker cluster) ditunda hingga fase berikutnya.

### 6. Matriks Pengujian Kualitas (Testing Protocol)
Setiap modul domain, penambahan endpoint, dan skrip migrasi wajib divalidasi melalui:
- **Smoke Tests:** Verifikasi koneksi database (Postgres ping, Redis ping, MinIO bucket check, Mailpit reachability).
- **Unit & Integration Tests:** Validasi logika bisnis domain, siklus otentikasi JWT/Argon2id, verifikasi OTP, dan kueri SeaORM.
- **Target SLA Pengujian:**
  - Endpoint REST API umum: Latensi p95 < 50ms.
  - Pencarian Katalog Leksikal FTS: Latensi < 5ms.
  - Scoped Semantic Quote Finder: Latensi < 10ms.
  - Paginasi Reader Leptos WASM: Render 60 FPS pada viewport layar.

### 7. Kebijakan Bebas Emoji & Nada Profesional (Mandatory)
- DILARANG menyertakan emoji, simbol emotikon grafis, atau gaya bahasa santai/slop di dalam kode sumber, komentar, docstrings, commit message, maupun dokumen teknis.
- Seluruh dokumentasi, penjelasan kode, dan laporan status wajib disampaikan secara profesional, formal, terstruktur, dan presisi.

### 8. Integritas Tautan Repositori (Repository Link Integrity)
- Dalam seluruh dokumentasi dan catatan commit, dilarang merujuk path absolut lokal mesin pengembang (misalnya `/home/aprxty3/...` atau `C:\...`).
- Rujuk berkas proyek selalu menggunakan tautan relatif yang bersih (misalnya `knowledge/prd.md` atau `crates/server/src/main.rs`).
