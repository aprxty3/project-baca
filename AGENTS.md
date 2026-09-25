---
type: Agent Guidelines
title: "Project Baca — Panduan & Aturan AI Agent"
description: "Aturan instruksi, batasan arsitektur, dan protokol eksekusi untuk seluruh coding agent yang beroperasi di repositori Project Baca."
tags: [agents, ai-instructions, conventions, governance, okf]
---

# AGENTS.md — Panduan & Tata Kelola Agen AI

Dokumen ini merupakan aturan operasional baku bagi seluruh asisten dan agen kecerdasan buatan (*AI coding agents*: Antigravity, Claude Code, Gemini CLI, Cursor, dll.) yang bekerja di dalam repositori **Project Baca**.

---

## 1. Sumber Kebenaran Tunggal (Single Source of Truth)

1. **Katalog Pengetahuan OKF v0.2:**
   - Direktori `knowledge/` adalah Sumber Kebenaran Tunggal (*SSOT*) untuk seluruh spesifikasi produk dan teknis.
   - Sebelum melakukan penalaran arsitektural atau menulis kode, agen **wajib** berkonsultasi dengan [knowledge/index.md](knowledge/index.md).
2. **Navigasi Graf Pengetahuan Graphify:**
   - Gunakan `graphify query "<pertanyaan>"`, `shortest_path`, atau `get_node` daripada membaca berkas mentah berukuran besar secara sporadis.
   - Periksa ringkasan komunitas di [graphify-out/GRAPH_REPORT.md](graphify-out/GRAPH_REPORT.md).

---

## 2. Core Invariants (Non-Negotiable)

1. **Zero Panics di Rust:**
   - Dilarang menggunakan `unwrap()` atau `expect()` pada jalur kode produksi (`crates/server`, `crates/domain`, `crates/infra`, `crates/shared`, `crates/web`).
   - Seluruh potensi kegagalan wajib ditangani menggunakan `Result<T, AppError>` secara terstruktur dan terisolasi.
2. **Postgres for Everything:**
   - Pencarian leksikal katalog dilayani oleh PostgreSQL 17 FTS + `pg_trgm` GIN index (<3ms).
   - Pencarian kutipan semantik dilayani oleh `pgvector` HNSW index (<10ms).
   - Dilarang menambahkan Elasticsearch demi efisiensi memori (menghemat 2–4 GB RAM) dan mencegah kompleksitas *dual-write*.
3. **Pragmatisme Message Broker:**
   - Antrean ingestion EPUB dan pengiriman email transaksional dilayani oleh Redis Streams.
   - Dilarang mengintroduksi broker terpisah seperti RedPanda atau RabbitMQ pada fase MVP.
4. **Manajemen Migrasi SQL Berpasangan:**
   - Seluruh perubahan skema database wajib dikelola melalui direktori `migrations/` dengan berkas SQL berpasangan: `<timestamp>_<nama>.up.sql` dan `<timestamp>_<nama>.down.sql`.
5. **Otomasi Terpusat melalui Makefile:**
   - Seluruh perintah build, run, test, dan migrasi wajib dapat diakses melalui berkas `Makefile` (`make dev`, `make db-up`, `make migrate-up`, dll).
6. **Estetika Vintage Literary (1900–1950):**
   - Antarmuka web Leptos WASM wajib merefleksikan keanggunan era cetak klasik (palet aged paper `#F9F6F0`, tinta cetak pekat `#2B2625`, aksen terakota `#9D5A3C`, tipografi serif klasik) tanpa elemen visual digital modern yang mencolok.

---

## 3. Prinsip Rekayasa Perangkat Lunak Baku

Setiap kontribusi agen wajib mematuhi 6 pilar berikut secara mutlak:

* **ROBUST:** Menangani seluruh kemungkinan kegagalan (koneksi database putus, rate limit tercapai, token kedaluwarsa, file EPUB korup) secara elegan tanpa crash.
* **SCALABLE:** Backend Axum stateless, pencarian kutipan semantik terisolasi per buku (`WHERE book_id = $1`), dan partial index GIN hanya pada baris `status = 'published'`.
* **EASY TO MAINTAIN:** Pemisahan tanggung jawab yang tegas antar-crate (`domain`, `shared`, `infra`, `server`, `web`), dokumentasi terstruktur OKF v0.2, dan struktur kode modular.
* **DRY (Don't Repeat Yourself):** DTOs dan logika validasi bersama diletakkan di `crates/shared` agar dapat digunakan oleh backend Axum maupun frontend Leptos WASM.
* **KISS (Keep It Simple, Stupid):** Pilih solusi paling sederhana yang menyelesaikan masalah secara andal tanpa layer abstraksi artifisial yang berlebihan.
* **YAGNI (You Aren't Gonna Need It):** Implementasikan hanya fitur yang telah dispesifikasikan dalam lingkup MVP saat ini. Fitur masa depan (seperti aplikasi mobile native, distributed worker cluster) ditunda hingga fase berikutnya.

---

## 4. Protokol Pencatatan Audit Trail

Setiap kali menyelesaikan perubahan kode yang bermakna, refaktor, perbaikan bug, atau pembaruan skema database:
1. Catat entri baru pada [knowledge/log.md](knowledge/log.md) dengan mencantumkan aktor, tanggal, aksi spesifik, dan konsep/dokumen yang terdampak (*affected concepts*).
2. Perbarui [MEMORY.md](MEMORY.md) untuk mendokumentasikan alasan arsitektural (*why*).
3. Mutakhirkan [PROJECT_LOG.md](PROJECT_LOG.md) untuk status operasional backlog.
4. Jalankan perintah `graphify update .` atau `graphify cluster-only` untuk menjaga sinkronisasi graf pengetahuan.

---

## 5. Kebijakan Bebas Emoji & Integritas Tautan

* **Kebijakan Bebas Emoji (Zero Emoji Policy):** DILARANG menyertakan emoji, simbol emotikon grafis, atau gaya bahasa informal di dalam kode sumber, komentar, docstrings, commit message, maupun dokumen teknis.
* **Integritas Tautan Repositori:** Dilarang menggunakan path absolut lokal mesin pengembang (misalnya `/home/aprxty3/...` atau `C:\...`). Selalu gunakan tautan markdown relatif bersih (misalnya `knowledge/prd.md`).
