# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 238 nodes · 270 edges · 55 communities (18 shown, 37 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 5 edges (avg confidence: 0.87)
- Token cost: 2,323 input · 622 output

## Graph Freshness
- Built from commit: `2cebb3cf`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Shared Data Transfer Objects
- Infrastructure and App Initialization
- Domain Entities and Logic
- Infrastructure and Database Setup
- Frontend Web Reader Development
- Project Roadmap and Stack
- Ingestion Pipeline and Admin
- Quality Assurance and Launch
- Authentication and Security System
- Semantic AI and Insights
- Catalog and Reader Backend
- AI Agent and Knowledge Guidelines
- Database Migration Scripts
- Project Crate Structure
- Project Design Documentation
- Frontend Component Logic
- Auth and Security Endpoints
- Search and Progress Flow
- Knowledge Graph Protocols
- Architecture Design Reference
- UX Flow and Navigation
- Distributed Infrastructure Configuration
- Offline Mode Support
- Performance and Search Metrics
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
- Project Overview Documentation

## God Nodes (most connected - your core abstractions)
1. `2. Rincian Langkah Kerja (Sub-Tasks)` - 8 edges
2. `knowledge/tasks/README.md — Master Task Roadmap` - 8 edges
3. `main()` - 7 edges
4. `Book` - 7 edges
5. `2. Rincian Langkah Kerja (Sub-Tasks)` - 7 edges
6. `2. Rincian Langkah Kerja (Sub-Tasks)` - 7 edges
7. `BookDetailDto` - 6 edges
8. `AppConfig` - 6 edges
9. `init_db_pool()` - 6 edges
10. `init_redis_client()` - 6 edges

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
- **MVP Implementation Pipeline** — knowledge_tasks_01_infrastructure_and_config_md, knowledge_tasks_02_authentication_and_user_md, knowledge_tasks_03_catalog_and_reader_backend_md, knowledge_tasks_04_semantic_ai_and_insights_md, knowledge_tasks_05_ingestion_pipeline_and_admin_md, knowledge_tasks_06_frontend_leptos_web_reader_md, knowledge_tasks_07_quality_assurance_and_launch_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **Semantic Intelligence Cluster** — knowledge_srs_quotes_search, knowledge_srs_atomic_cards, knowledge_srs_chapter_recap [EXTRACTED 1.00]
- **Core Reading Experience** — knowledge_srs_books_chapter, knowledge_srs_progress_sync, knowledge_srs_chapter_recap, knowledge_ux_flow_philosophy [INFERRED 0.90]

## Communities (55 total, 37 thin omitted)

### Community 0 - "Shared Data Transfer Objects"
Cohesion: 0.16
Nodes (23): ApiResponse, ApiResponse<T>, BookDetailDto, BookSummaryDto, ChapterSummaryDto, ErrorPayload, LoginRequest, QuoteSearchRequest (+15 more)

### Community 1 - "Infrastructure and App Initialization"
Cohesion: 0.13
Nodes (22): Arc, Box, AppConfig, init_db_pool(), init_redis_client(), Client, DatabaseConnection, Result (+14 more)

### Community 2 - "Domain Entities and Logic"
Cohesion: 0.23
Nodes (17): Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak, DateTime (+9 more)

### Community 3 - "Infrastructure and Database Setup"
Cohesion: 0.12
Nodes (14): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 1.1: Integrasi Pustaka `dotenvy` & Pengaturan Dependensi, Sub-Task 1.2: Restrukturisasi `AppConfig` Menjadi Sub-Konfigurasi Modular, Sub-Task 1.3: Dinamisasi Pool Koneksi Basis Data & Redis, Sub-Task 1.4: Pemodelan Entitas SeaORM untuk 13 Tabel Domain, Sub-Task 1.5: Modularisasi `AppState` pada Server (+6 more)

### Community 4 - "Frontend Web Reader Development"
Cohesion: 0.17
Nodes (11): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 6.1: Sistem Desain CSS Vintage Literary (1900–1950), Sub-Task 6.2: Layanan Komunikasi API & Local-First IndexedDB (`rexie`), Sub-Task 6.3: Halaman Beranda (Discovery Catalog & Search), Sub-Task 6.4: Halaman Sinopsis Buku (Book Overview), Sub-Task 6.5: Komponen Pembaca Naskah Reflowable (Reflowable E-Reader) (+3 more)

### Community 5 - "Project Roadmap and Stack"
Cohesion: 0.18
Nodes (11): Task 01: Infrastructure & Config, Task 02: Authentication & User, Task 03: Catalog & Reader Backend, Task 04: Semantic AI & Insights, Task 05: Ingestion Pipeline & Admin, Task 06: Frontend Leptos Web Reader, Task 07: QA & Launch, knowledge/tasks/README.md — Master Task Roadmap (+3 more)

### Community 6 - "Ingestion Pipeline and Admin"
Cohesion: 0.18
Nodes (10): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 5.1: Layanan Penyimpanan Objek S3/MinIO (`crates/infra`), Sub-Task 5.2: Endpoint Unggah EPUB Multi-Part Admin (SRS 21), Sub-Task 5.3: Pekerja Latar Ingestion & Parsing EPUB, Sub-Task 5.4: Sanitasi HTML Naskah Klasik & Scene Chunking, Sub-Task 5.5: Ekstraksi Vektor 768-Dim & Generasi Wawasan Awal (+2 more)

### Community 7 - "Quality Assurance and Launch"
Cohesion: 0.18
Nodes (10): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 7.1: Pengujian Asap Infrastruktur (Smoke Testing), Sub-Task 7.2: Pengujian Unit Crate Bersama & Domain (Unit Test Suite), Sub-Task 7.3: Pengujian Integrasi Skenario Kritis (Integration Test Suite), Sub-Task 7.4: Pengukuran & Benchmarking Ambang Batas SLA, Sub-Task 7.5: Audit Keamanan & Hardening Pra-Rilis (+2 more)

### Community 8 - "Authentication and Security System"
Cohesion: 0.20
Nodes (9): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 2.1: Enkripsi Kata Sandi (Argon2id) & Modul Hashing OTP, Sub-Task 2.2: Pengelolaan Token JWT & Rotasi Refresh Token, Sub-Task 2.3: Implementasi Endpoint REST API Otentikasi (SRS 1–7), Sub-Task 2.4: Endpoint Rekonsiliasi Otomatis Data Tamu (`POST /api/progress/merge`), Sub-Task 2.5: Middleware Lapisan Keamanan Axum (+1 more)

### Community 9 - "Semantic AI and Insights"
Cohesion: 0.20
Nodes (9): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 4.1: Layanan Dual-Mode Embedding Provider (`crates/infra`), Sub-Task 4.2: Scoped Semantic Quote Finder (`POST /api/books/{id}/quotes/search`), Sub-Task 4.3: Pengelolaan Cache Kartu Wawasan Atomik (SRS 18), Sub-Task 4.4: Rekap Kontekstual Anti-Spoiler Bab Sebelumnya (SRS 19), Sub-Task 4.5: Penandaan & Pengelolaan Kutipan Favorit (SRS 20) (+1 more)

### Community 10 - "Catalog and Reader Backend"
Cohesion: 0.22
Nodes (8): 1. Ringkasan Tugas, 2. Rincian Langkah Kerja (Sub-Tasks), 3. Kriteria Keberhasilan & Validasi, Sub-Task 3.1: Layanan Repository Katalog & Pencarian Leksikal (`crates/infra`), Sub-Task 3.2: Implementasi Endpoint REST API Katalog & Naskah Bab (SRS 8–12), Sub-Task 3.3: Layanan Progres Membaca & Sinkronisasi CFI (SRS 13–14), Sub-Task 3.4: Sistem Detak Jantung Aktivitas Baca, Streak & Lencana (SRS 15–16), Task 03: Backend Katalog Buku, FTS Trigram, Reader API & Gamifikasi Streak

### Community 11 - "AI Agent and Knowledge Guidelines"
Cohesion: 0.29
Nodes (8): AGENTS.md — Panduan & Tata Kelola Agen AI, OKF & Persistent Memory Guidelines, CLAUDE.md — Claude Code Guidelines, GEMINI.md — Panduan Gemini & Google Antigravity, knowledge/index.md — Master Knowledge Catalog, knowledge/log.md — Audit Trail, MEMORY.md — Log Keputusan Arsitektur & Memori Sistem, Open Knowledge Format v0.2

### Community 12 - "Database Migration Scripts"
Cohesion: 0.76
Nodes (6): init_table(), migrate_down(), migrate_status(), migrate_up(), psql_cmd(), migrate.sh script

### Community 13 - "Project Crate Structure"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 14 - "Project Design Documentation"
Cohesion: 0.50
Nodes (4): AI Quote Finder Illustration, knowledge/erd.md — Entity Relationship Diagram, knowledge/frd.md — Functional Requirements Document, knowledge/prd.md — Product Requirements Document

### Community 16 - "Auth and Security Endpoints"
Cohesion: 0.67
Nodes (3): POST /api/auth/refresh, POST /api/auth/verify-otp, Security Architecture

### Community 17 - "Search and Progress Flow"
Cohesion: 0.67
Nodes (3): GET /api/books/search, POST /api/progress/merge, User Journey Flowchart

## Knowledge Gaps
- **112 isolated node(s):** `1. Ringkasan Tugas`, `3. Kriteria Keberhasilan & Validasi`, `Sub-Task 3.1: Layanan Repository Katalog & Pencarian Leksikal (`crates/infra`)`, `Sub-Task 3.2: Implementasi Endpoint REST API Katalog & Naskah Bab (SRS 8–12)`, `Sub-Task 3.3: Layanan Progres Membaca & Sinkronisasi CFI (SRS 13–14)` (+107 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **37 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Are the 2 inferred relationships involving `main()` (e.g. with `init_db_pool()` and `init_redis_client()`) actually correct?**
  _`main()` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `1. Ringkasan Tugas`, `3. Kriteria Keberhasilan & Validasi`, `Sub-Task 3.1: Layanan Repository Katalog & Pencarian Leksikal (`crates/infra`)` to the rest of the system?**
  _112 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Infrastructure and App Initialization` be split into smaller, more focused modules?**
  _Cohesion score 0.12666666666666668 - nodes in this community are weakly interconnected._
- **Should `Infrastructure and Database Setup` be split into smaller, more focused modules?**
  _Cohesion score 0.125 - nodes in this community are weakly interconnected._