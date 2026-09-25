# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 231 nodes · 267 edges · 57 communities (17 shown, 40 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 5 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2de6c25e`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Shared Data Transfer Objects
- Infrastructure and App Initialization
- README.md
- domain/src/lib.rs
- 2. Rincian Langkah Kerja (Sub-Tasks)
- 2. Rincian Langkah Kerja (Sub-Tasks)
- 2. Rincian Langkah Kerja (Sub-Tasks)
- 2. Rincian Langkah Kerja (Sub-Tasks)
- 2. Rincian Langkah Kerja (Sub-Tasks)
- knowledge/index.md — Master Knowledge Catalog
- 2. Rincian Langkah Kerja (Sub-Tasks)
- migrate.sh
- shared
- knowledge/frd.md — Functional Requirements Document
- App
- Security Architecture
- User Journey Flowchart
- Graphify Knowledge Graph Protocol
- ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain
- Admin Sorting Illustration
- DISTRIBUTED.md — Arsitektur Pemrosesan Terdistribusi
- GET /api/books/{id}/offline-bundle
- Performance Metrics & SLA
- knowledge/tasks/README.md — Master Task Roadmap
- Reader Interface Design
- Library Catalog Design
- Ingestion Monitoring Interface
- Asset Management
- Project Changelog
- Web Entry Point
- Developer Operations Guide
- SMTP Development Service
- Object Storage Service
- Vector Database Infrastructure
- Cache Infrastructure
- Activity Heartbeat Endpoint
- Ingestion Job Monitoring
- Book Upload Endpoint
- API Error Standards
- Atomic Insights Endpoint
- User Login Endpoint
- User Logout Endpoint
- User Signup Endpoint
- Chapter Content Endpoint
- Book Detail Endpoint
- Book Catalog Endpoint
- Contextual Recap Endpoint
- Software Requirements Specification
- User Account Deletion
- User Profile Endpoint
- Reading Progress Update
- Core UX Philosophy
- Legal and Licensing
- PostgreSQL 17 + pgvector
- README.md — Project Baca Overview
- Redis 7

## God Nodes (most connected - your core abstractions)
1. `2. Rincian Langkah Kerja (Sub-Tasks)` - 8 edges
2. `main()` - 7 edges
3. `Book` - 7 edges
4. `2. Rincian Langkah Kerja (Sub-Tasks)` - 7 edges
5. `2. Rincian Langkah Kerja (Sub-Tasks)` - 7 edges
6. `BookDetailDto` - 6 edges
7. `AppConfig` - 6 edges
8. `init_db_pool()` - 6 edges
9. `init_redis_client()` - 6 edges
10. `AppError` - 6 edges

## Surprising Connections (you probably didn't know these)
- `AI Quote Finder Illustration` --conceptually_related_to--> `knowledge/prd.md — Product Requirements Document`  [EXTRACTED]
  assets/illustrations/retro-rocket-discovery.png → knowledge/prd.md
- `GBrain Design Reference` --conceptually_related_to--> `ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain`  [EXTRACTED]
  assets/references/gbrain-design-reference.png → ARCHITECTURE.md
- `Admin Sorting Illustration` --conceptually_related_to--> `knowledge/ux-flow.md — UX Flow & Navigation`  [EXTRACTED]
  assets/illustrations/admin-sorting-pigeonholes.png → knowledge/ux-flow.md
- `DISTRIBUTED.md — Arsitektur Pemrosesan Terdistribusi` --references--> `Docker Infrastructure Configuration`  [INFERRED]
  DISTRIBUTED.md → docker-compose.yml
- `PROJECT_LOG.md — Log & Memory Proyek` --references--> `knowledge/tasks/README.md — Master Task Roadmap`  [EXTRACTED]
  PROJECT_LOG.md → knowledge/tasks/README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Authentication & Account Management Flow** — knowledge_srs_auth_signup, knowledge_srs_auth_verify_otp, knowledge_srs_auth_login, knowledge_srs_auth_refresh, knowledge_srs_auth_logout [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **MVP Development Critical Path** — knowledge_tasks_01_infrastructure_and_config, knowledge_tasks_02_authentication_and_user, knowledge_tasks_03_catalog_and_reader_backend, knowledge_tasks_04_semantic_ai_and_insights, knowledge_tasks_05_ingestion_pipeline_and_admin, knowledge_tasks_06_frontend_leptos_web_reader, knowledge_tasks_07_quality_assurance_and_launch [EXTRACTED 1.00]
- **Semantic Intelligence Cluster** — knowledge_srs_quotes_search, knowledge_srs_atomic_cards, knowledge_srs_chapter_recap [EXTRACTED 1.00]
- **Core Reading Experience** — knowledge_srs_books_chapter, knowledge_srs_progress_sync, knowledge_srs_chapter_recap, knowledge_ux_flow_philosophy [INFERRED 0.90]

## Communities (57 total, 40 thin omitted)

### Community 0 - "Shared Data Transfer Objects"
Cohesion: 0.16
Nodes (23): ApiResponse, ApiResponse<T>, BookDetailDto, BookSummaryDto, ChapterSummaryDto, ErrorPayload, LoginRequest, QuoteSearchRequest (+15 more)

### Community 1 - "Infrastructure and App Initialization"
Cohesion: 0.13
Nodes (22): Arc, Box, AppConfig, init_db_pool(), init_redis_client(), Client, DatabaseConnection, Result (+14 more)

### Community 2 - "README.md"
Cohesion: 0.12
Nodes (14): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 1.1: Integrasi Pustaka `dotenvy` & Pengaturan Dependensi, Sub-Task 1.2: Restrukturisasi `AppConfig` Menjadi Sub-Konfigurasi Modular, Sub-Task 1.3: Dinamisasi Pool Koneksi Basis Data & Redis, Sub-Task 1.4: Pemodelan Entitas SeaORM untuk 13 Tabel Domain, Sub-Task 1.5: Modularisasi `AppState` pada Server (+6 more)

### Community 3 - "domain/src/lib.rs"
Cohesion: 0.23
Nodes (17): Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak, DateTime (+9 more)

### Community 4 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.18
Nodes (11): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 6.1: Sistem Desain CSS Vintage Literary (1900–1950), Sub-Task 6.2: Layanan Komunikasi API & Local-First IndexedDB (`rexie`), Sub-Task 6.3: Halaman Beranda (Discovery Catalog & Search), Sub-Task 6.4: Halaman Sinopsis Buku (Book Overview), Sub-Task 6.5: Komponen Pembaca Naskah Reflowable (Reflowable E-Reader) (+3 more)

### Community 5 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.20
Nodes (10): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 5.1: Layanan Penyimpanan Objek S3/MinIO (`crates/infra`), Sub-Task 5.2: Endpoint Unggah EPUB Multi-Part Admin (SRS 21), Sub-Task 5.3: Pekerja Latar Ingestion & Parsing EPUB, Sub-Task 5.4: Sanitasi HTML Naskah Klasik & Scene Chunking, Sub-Task 5.5: Ekstraksi Vektor 768-Dim & Generasi Wawasan Awal (+2 more)

### Community 6 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.20
Nodes (10): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 7.1: Pengujian Asap Infrastruktur (Smoke Testing), Sub-Task 7.2: Pengujian Unit Crate Bersama & Domain (Unit Test Suite), Sub-Task 7.3: Pengujian Integrasi Skenario Kritis (Integration Test Suite), Sub-Task 7.4: Pengukuran & Benchmarking Ambang Batas SLA, Sub-Task 7.5: Audit Keamanan & Hardening Pra-Rilis (+2 more)

### Community 7 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.22
Nodes (9): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 2.1: Enkripsi Kata Sandi (Argon2id) & Modul Hashing OTP, Sub-Task 2.2: Pengelolaan Token JWT & Rotasi Refresh Token, Sub-Task 2.3: Implementasi Endpoint REST API Otentikasi (SRS 1–7), Sub-Task 2.4: Endpoint Rekonsiliasi Otomatis Data Tamu (`POST /api/progress/merge`), Sub-Task 2.5: Middleware Lapisan Keamanan Axum (+1 more)

### Community 8 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.22
Nodes (9): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 4.1: Layanan Dual-Mode Embedding Provider (`crates/infra`), Sub-Task 4.2: Scoped Semantic Quote Finder (`POST /api/books/{id}/quotes/search`), Sub-Task 4.3: Pengelolaan Cache Kartu Wawasan Atomik (SRS 18), Sub-Task 4.4: Rekap Kontekstual Anti-Spoiler Bab Sebelumnya (SRS 19), Sub-Task 4.5: Penandaan & Pengelolaan Kutipan Favorit (SRS 20) (+1 more)

### Community 9 - "knowledge/index.md — Master Knowledge Catalog"
Cohesion: 0.29
Nodes (8): AGENTS.md — Panduan & Tata Kelola Agen AI, OKF & Persistent Memory Guidelines, CLAUDE.md — Claude Code Guidelines, GEMINI.md — Panduan Gemini & Google Antigravity, knowledge/index.md — Master Knowledge Catalog, knowledge/log.md — Audit Trail, MEMORY.md — Log Keputusan Arsitektur & Memori Sistem, Open Knowledge Format v0.2

### Community 10 - "2. Rincian Langkah Kerja (Sub-Tasks)"
Cohesion: 0.25
Nodes (8): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 3.1: Layanan Repository Katalog & Pencarian Leksikal (`crates/infra`), Sub-Task 3.2: Implementasi Endpoint REST API Katalog & Naskah Bab (SRS 8–12), Sub-Task 3.3: Layanan Progres Membaca & Sinkronisasi CFI (SRS 13–14), Sub-Task 3.4: Sistem Detak Jantung Aktivitas Baca, Streak & Lencana (SRS 15–16), Task 03: Backend Katalog Buku, FTS Trigram, Reader API & Gamifikasi Streak

### Community 11 - "migrate.sh"
Cohesion: 0.76
Nodes (6): init_table(), migrate_down(), migrate_status(), migrate_up(), psql_cmd(), migrate.sh script

### Community 12 - "shared"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 13 - "knowledge/frd.md — Functional Requirements Document"
Cohesion: 0.50
Nodes (4): AI Quote Finder Illustration, knowledge/erd.md — Entity Relationship Diagram, knowledge/frd.md — Functional Requirements Document, knowledge/prd.md — Product Requirements Document

### Community 15 - "Security Architecture"
Cohesion: 0.67
Nodes (3): POST /api/auth/refresh, POST /api/auth/verify-otp, Security Architecture

### Community 16 - "User Journey Flowchart"
Cohesion: 0.67
Nodes (3): GET /api/books/search, POST /api/progress/merge, User Journey Flowchart

## Knowledge Gaps
- **108 isolated node(s):** `1. Ringkasan Tugas`, `3. Kriteria Keberhasilan & Validasi`, `Sub-Task 3.1: Layanan Repository Katalog & Pencarian Leksikal (`crates/infra`)`, `Sub-Task 3.2: Implementasi Endpoint REST API Katalog & Naskah Bab (SRS 8–12)`, `Sub-Task 3.3: Layanan Progres Membaca & Sinkronisasi CFI (SRS 13–14)` (+103 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **40 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Task 06: Frontend Leptos WASM Web Reader & Mode Offline` connect `2. Rincian Langkah Kerja (Sub-Tasks)` to `README.md`?**
  _High betweenness centrality (0.026) - this node is a cross-community bridge._
- **Why does `Task 05: Ingestion Pipeline EPUB Asinkron & Admin Management` connect `2. Rincian Langkah Kerja (Sub-Tasks)` to `README.md`?**
  _High betweenness centrality (0.024) - this node is a cross-community bridge._
- **Why does `Task 07: Validasi Kualitas Menyeluruh, SLA Benchmarking & Kesiapan Rilis MVP` connect `2. Rincian Langkah Kerja (Sub-Tasks)` to `README.md`?**
  _High betweenness centrality (0.024) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `main()` (e.g. with `init_db_pool()` and `init_redis_client()`) actually correct?**
  _`main()` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `1. Ringkasan Tugas`, `3. Kriteria Keberhasilan & Validasi`, `Sub-Task 3.1: Layanan Repository Katalog & Pencarian Leksikal (`crates/infra`)` to the rest of the system?**
  _108 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Infrastructure and App Initialization` be split into smaller, more focused modules?**
  _Cohesion score 0.12666666666666668 - nodes in this community are weakly interconnected._
- **Should `README.md` be split into smaller, more focused modules?**
  _Cohesion score 0.11688311688311688 - nodes in this community are weakly interconnected._