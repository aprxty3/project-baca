# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- 55 files · ~200,031 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 234 nodes · 271 edges · 51 communities (15 shown, 36 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 5 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `29aaa7e0`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- shared/src/lib.rs
- Domain Entities
- server/src/main.rs
- Project Baca — Log Keputusan Arsitektur & Memori Sistem
- README.md
- 2. Rincian Langkah Kerja (Sub-Tasks)
- Database Migrations
- 2. Rincian Langkah Kerja (Sub-Tasks)
- Project Modules
- Web Frontend Library
- Authentication Security
- User Progress Flow
- Knowledge Graph Protocol
- System Design Reference
- UX Flow Design
- Offline Bundle API
- Performance and Search
- UI Design Style
- Reader Interface Illustration
- Library Catalog Assets
- Data Ingestion Monitoring
- Asset Catalog
- Project Changelog
- 2. Rincian Langkah Kerja (Sub-Tasks)
- 2. Rincian Langkah Kerja (Sub-Tasks)
- 2. Rincian Langkah Kerja (Sub-Tasks)
- Web Entry Point
- Developer Guide
- Mail Server Development
- Object Storage
- Vector Database
- Redis Cache
- Activity Heartbeat API
- Ingestion Job Status
- Book Upload API
- API Error Standards
- Atomic Cards API
- User Login API
- User Logout API
- User Signup API
- Chapter Content API
- Book Detail API
- Book List API
- Chapter Recap API
- Delete Account API
- User Profile API
- Update Progress API
- UX Philosophy
- Legal and License
- Project Overview

## God Nodes (most connected - your core abstractions)
1. `2. Rincian Langkah Kerja (Sub-Tasks)` - 8 edges
2. `Project Baca — Log Keputusan Arsitektur & Memori Sistem` - 8 edges
3. `Book` - 7 edges
4. `main()` - 7 edges
5. `2. Rincian Langkah Kerja (Sub-Tasks)` - 7 edges
6. `2. Rincian Langkah Kerja (Sub-Tasks)` - 7 edges
7. `knowledge/index.md — Master Knowledge Catalog` - 7 edges
8. `User` - 6 edges
9. `AppConfig` - 6 edges
10. `init_db_pool()` - 6 edges

## Surprising Connections (you probably didn't know these)
- `GBrain Design Reference` --conceptually_related_to--> `ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain`  [EXTRACTED]
  assets/references/gbrain-design-reference.png → ARCHITECTURE.md
- `Admin Sorting Illustration` --conceptually_related_to--> `knowledge/ux-flow.md — UX Flow & Navigation`  [EXTRACTED]
  assets/illustrations/admin-sorting-pigeonholes.png → knowledge/ux-flow.md
- `AI Quote Finder Illustration` --conceptually_related_to--> `knowledge/prd.md — Product Requirements Document`  [EXTRACTED]
  assets/illustrations/retro-rocket-discovery.png → knowledge/prd.md
- `CLAUDE.md — Claude Code Guidelines` --references--> `knowledge/index.md — Master Knowledge Catalog`  [EXTRACTED]
  CLAUDE.md → knowledge/index.md
- `GEMINI.md — Panduan Gemini & Google Antigravity` --references--> `knowledge/index.md — Master Knowledge Catalog`  [EXTRACTED]
  GEMINI.md → knowledge/index.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Authentication & Account Management Flow** — knowledge_srs_auth_signup, knowledge_srs_auth_verify_otp, knowledge_srs_auth_login, knowledge_srs_auth_refresh, knowledge_srs_auth_logout [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **Semantic Intelligence Cluster** — knowledge_srs_quotes_search, knowledge_srs_atomic_cards, knowledge_srs_chapter_recap [EXTRACTED 1.00]
- **Data Persistence & Search Stack** — infra_postgres_17, arch_postgres_everything, arch_dual_mode_embedding [INFERRED 0.85]
- **Core Reading Experience** — knowledge_srs_books_chapter, knowledge_srs_progress_sync, knowledge_srs_chapter_recap, knowledge_ux_flow_philosophy [INFERRED 0.90]

## Communities (51 total, 36 thin omitted)

### Community 0 - "shared/src/lib.rs"
Cohesion: 0.16
Nodes (23): ApiResponse, ApiResponse<T>, BookDetailDto, BookSummaryDto, ChapterSummaryDto, ErrorPayload, LoginRequest, QuoteSearchRequest (+15 more)

### Community 1 - "Domain Entities"
Cohesion: 0.23
Nodes (17): Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak, DateTime (+9 more)

### Community 2 - "server/src/main.rs"
Cohesion: 0.13
Nodes (22): Arc, Box, AppConfig, init_db_pool(), init_redis_client(), Client, DatabaseConnection, Result (+14 more)

### Community 3 - "Project Baca — Log Keputusan Arsitektur & Memori Sistem"
Cohesion: 0.11
Nodes (21): AGENTS.md — Panduan & Tata Kelola Agen AI, OKF & Persistent Memory Guidelines, Dual-Mode Embedding (768-Dim), Monorepo Polyglot Architecture, Open Knowledge Format v0.2, Postgres for Everything, Redis Streams Broker, 6 Engineering Pillars (+13 more)

### Community 4 - "README.md"
Cohesion: 0.08
Nodes (23): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 1.1: Integrasi Pustaka `dotenvy` & Pengaturan Dependensi, Sub-Task 1.2: Restrukturisasi `AppConfig` Menjadi Sub-Konfigurasi Modular, Sub-Task 1.3: Dinamisasi Pool Koneksi Basis Data & Redis, Sub-Task 1.4: Pemodelan Entitas SeaORM untuk 13 Tabel Domain, Sub-Task 1.5: Modularisasi `AppState` pada Server (+15 more)

### Community 5 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.17
Nodes (11): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 6.1: Sistem Desain CSS Vintage Literary (1900–1950), Sub-Task 6.2: Layanan Komunikasi API & Local-First IndexedDB (`rexie`), Sub-Task 6.3: Halaman Beranda (Discovery Catalog & Search), Sub-Task 6.4: Halaman Sinopsis Buku (Book Overview), Sub-Task 6.5: Komponen Pembaca Naskah Reflowable (Reflowable E-Reader) (+3 more)

### Community 6 - "Database Migrations"
Cohesion: 0.76
Nodes (6): init_table(), migrate_down(), migrate_status(), migrate_up(), psql_cmd(), migrate.sh script

### Community 7 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.18
Nodes (10): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 5.1: Layanan Penyimpanan Objek S3/MinIO (`crates/infra`), Sub-Task 5.2: Endpoint Unggah EPUB Multi-Part Admin (SRS 21), Sub-Task 5.3: Pekerja Latar Ingestion & Parsing EPUB, Sub-Task 5.4: Sanitasi HTML Naskah Klasik & Scene Chunking, Sub-Task 5.5: Ekstraksi Vektor 768-Dim & Generasi Wawasan Awal (+2 more)

### Community 8 - "Project Modules"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 10 - "Authentication Security"
Cohesion: 0.67
Nodes (3): POST /api/auth/refresh, POST /api/auth/verify-otp, Security Architecture

### Community 11 - "User Progress Flow"
Cohesion: 0.67
Nodes (3): GET /api/books/search, POST /api/progress/merge, User Journey Flowchart

### Community 24 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.18
Nodes (10): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 7.1: Pengujian Asap Infrastruktur (Smoke Testing), Sub-Task 7.2: Pengujian Unit Crate Bersama & Domain (Unit Test Suite), Sub-Task 7.3: Pengujian Integrasi Skenario Kritis (Integration Test Suite), Sub-Task 7.4: Pengukuran & Benchmarking Ambang Batas SLA, Sub-Task 7.5: Audit Keamanan & Hardening Pra-Rilis (+2 more)

### Community 25 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.20
Nodes (9): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 4.1: Layanan Dual-Mode Embedding Provider (`crates/infra`), Sub-Task 4.2: Scoped Semantic Quote Finder (`POST /api/books/{id}/quotes/search`), Sub-Task 4.3: Pengelolaan Cache Kartu Wawasan Atomik (SRS 18), Sub-Task 4.4: Rekap Kontekstual Anti-Spoiler Bab Sebelumnya (SRS 19), Sub-Task 4.5: Penandaan & Pengelolaan Kutipan Favorit (SRS 20) (+1 more)

### Community 26 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.22
Nodes (8): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 3.1: Layanan Repository Katalog & Pencarian Leksikal (`crates/infra`), Sub-Task 3.2: Implementasi Endpoint REST API Katalog & Naskah Bab (SRS 8–12), Sub-Task 3.3: Layanan Progres Membaca & Sinkronisasi CFI (SRS 13–14), Sub-Task 3.4: Sistem Detak Jantung Aktivitas Baca, Streak & Lencana (SRS 15–16), Task 03: Backend Katalog Buku, FTS Trigram, Reader API & Gamifikasi Streak

## Knowledge Gaps
- **109 isolated node(s):** `web`, `1. Ringkasan Tugas`, `Sub-Task 1.1: Integrasi Pustaka `dotenvy` & Pengaturan Dependensi`, `Sub-Task 1.2: Restrukturisasi `AppConfig` Menjadi Sub-Konfigurasi Modular`, `Sub-Task 1.3: Dinamisasi Pool Koneksi Basis Data & Redis` (+104 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **36 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What connects `web`, `1. Ringkasan Tugas`, `Sub-Task 1.1: Integrasi Pustaka `dotenvy` & Pengaturan Dependensi` to the rest of the system?**
  _109 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `server/src/main.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12666666666666668 - nodes in this community are weakly interconnected._
- **Should `Project Baca — Log Keputusan Arsitektur & Memori Sistem` be split into smaller, more focused modules?**
  _Cohesion score 0.10952380952380952 - nodes in this community are weakly interconnected._
- **Should `README.md` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._