---
type: Agent Guidelines
title: "Project Baca — Gemini Instructions & Guidelines"
description: "Petunjuk operasional baku untuk Google Antigravity dan model Gemini yang beroperasi di repositori Project Baca."
tags: [gemini, antigravity, guidelines, instructions, okf]
---

# GEMINI.md — Panduan Gemini & Google Antigravity

Dokumen ini memuat instruksi operasional untuk asisten **Google Antigravity** dan model **Gemini** saat beroperasi di repositori **Project Baca**.

---

## 1. Protokol Navigasi & Sumber Kebenaran (SSOT)

* **OKF v0.2 Master Catalog:** Sebelum mengeksplorasi kode mentah atau menjawab pertanyaan arsitektur yang kompleks, selalu baca [knowledge/index.md](knowledge/index.md).
* **Graphify Knowledge Graph:** Gunakan alat `query_graph`, `shortest_path`, atau CLI `graphify query` untuk memeriksa relasi antarmodul dan dokumentasi di direktori `graphify-out/`.
* **Progressive Disclosure:**
  - Lapis 1: [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md), [knowledge/index.md](knowledge/index.md).
  - Lapis 2: [knowledge/prd.md](knowledge/prd.md), [knowledge/ux-flow.md](knowledge/ux-flow.md).
  - Lapis 3: [knowledge/frd.md](knowledge/frd.md), [knowledge/erd.md](knowledge/erd.md), [knowledge/srs.md](knowledge/srs.md).
  - Lapis 4: [knowledge/log.md](knowledge/log.md), [MEMORY.md](MEMORY.md), [PROJECT_LOG.md](PROJECT_LOG.md).

---

## 2. Invarian Rekayasa Non-Negotiable

1. **Rust Zero Panics:**
   - DILARANG menggunakan `.unwrap()` atau `.expect()` pada jalur kode produksi.
   - Gunakan `Result<T, AppError>` untuk seluruh kemungkinan kegagalan I/O, database, atau parsing.
2. **Postgres for Everything:**
   - Pencarian leksikal: PostgreSQL 17 Full-Text Search + `pg_trgm` GIN index (<3ms).
   - Pencarian semantik: PostgreSQL 17 `pgvector` HNSW index (<10ms).
   - Penolakan Elasticsearch untuk menghemat RAM dan mencegah masalah dual-write.
3. **Pragmatisme Message Broker:**
   - Gunakan antrean **Redis Streams** untuk ingestion EPUB dan pengiriman email.
   - Dilarang menambahkan RabbitMQ atau RedPanda pada fase MVP.
4. **SeaORM & Migrasi SQL Berpasangan:**
   - Skema database dikelola melalui file SQL berpasangan di `migrations/` (`.up.sql` dan `.down.sql`).
   - Otomasi dikendalikan via `Makefile` (`make migrate-up`, `make db-up`, dll).
5. **Estetika Vintage Literary (1900–1950):**
   - Leptos WASM frontend menggunakan palet kertas kuno, tinta cetak klasik, aksen terakota, dan tipografi serif sastrawan.

---

## 3. Disiplin Rekayasa & Kebijakan Nol Emoji

* **ROBUST, SCALABLE, EASY TO MAINTAIN, DRY, KISS, YAGNI:** Wajib dipatuhi pada setiap modifikasi berkas.
* **Kebijakan Bebas Emoji (Zero Emoji Policy):** DILARANG menyertakan emoji atau simbol emotikon grafis dalam kode, komentar, docstrings, commit message, maupun dokumentasi.
* **Pencatatan Audit Trail:** Setelah menyelesaikan perubahan kode atau skema yang bermakna, catat pembaruan di [knowledge/log.md](knowledge/log.md) dan [MEMORY.md](MEMORY.md), lalu sinkronkan graf via `graphify update .`.
