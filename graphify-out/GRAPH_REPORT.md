# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 161 nodes · 193 edges · 53 communities (12 shown, 41 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 5 edges (avg confidence: 0.87)
- Token cost: 1,057 input · 453 output

## Graph Freshness
- Built from commit: `8bec6b84`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Data Transfer Objects
- Domain Entities
- Server Entry Point
- Project Documentation
- Infrastructure Configuration
- Architecture Guidelines
- Database Migrations
- API Response Wrapper
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
- Client Type
- Database Connection Type
- Result Type
- Self Reference
- String Type
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
1. `Project Baca — Log Keputusan Arsitektur & Memori Sistem` - 8 edges
2. `Book` - 7 edges
3. `main()` - 7 edges
4. `knowledge/index.md — Master Knowledge Catalog` - 7 edges
5. `BookDetailDto` - 6 edges
6. `User` - 6 edges
7. `AppConfig` - 6 edges
8. `init_db_pool()` - 6 edges
9. `init_redis_client()` - 6 edges
10. `BookSummaryDto` - 5 edges

## Surprising Connections (you probably didn't know these)
- `GBrain Design Reference` --conceptually_related_to--> `ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain`  [EXTRACTED]
  assets/references/gbrain-design-reference.png → ARCHITECTURE.md
- `Admin Sorting Illustration` --conceptually_related_to--> `knowledge/ux-flow.md — UX Flow & Navigation`  [EXTRACTED]
  assets/illustrations/admin-sorting-pigeonholes.png → knowledge/ux-flow.md
- `AI Quote Finder Illustration` --conceptually_related_to--> `knowledge/prd.md — Product Requirements Document`  [EXTRACTED]
  assets/illustrations/retro-rocket-discovery.png → knowledge/prd.md
- `AGENTS.md — Panduan & Tata Kelola Agen AI` --references--> `Project Baca — Log Keputusan Arsitektur & Memori Sistem`  [EXTRACTED]
  AGENTS.md → MEMORY.md
- `CLAUDE.md — Claude Code Guidelines` --references--> `knowledge/index.md — Master Knowledge Catalog`  [EXTRACTED]
  CLAUDE.md → knowledge/index.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Authentication & Account Management Flow** — knowledge_srs_auth_signup, knowledge_srs_auth_verify_otp, knowledge_srs_auth_login, knowledge_srs_auth_refresh, knowledge_srs_auth_logout [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **Semantic Intelligence Cluster** — knowledge_srs_quotes_search, knowledge_srs_atomic_cards, knowledge_srs_chapter_recap [EXTRACTED 1.00]
- **Data Persistence & Search Stack** — infra_postgres_17, arch_postgres_everything, arch_dual_mode_embedding [INFERRED 0.85]
- **Core Reading Experience** — knowledge_srs_books_chapter, knowledge_srs_progress_sync, knowledge_srs_chapter_recap, knowledge_ux_flow_philosophy [INFERRED 0.90]

## Communities (53 total, 41 thin omitted)

### Community 0 - "Data Transfer Objects"
Cohesion: 0.21
Nodes (20): ApiResponse, AppError, BookDetailDto, BookSummaryDto, ChapterSummaryDto, ErrorPayload, LoginRequest, QuoteSearchRequest (+12 more)

### Community 1 - "Domain Entities"
Cohesion: 0.23
Nodes (17): Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak, DateTime (+9 more)

### Community 2 - "Server Entry Point"
Cohesion: 0.20
Nodes (13): Arc, Box, api_health_check(), AppState, health_check(), main(), Client, DatabaseConnection (+5 more)

### Community 3 - "Project Documentation"
Cohesion: 0.22
Nodes (11): AGENTS.md — Panduan & Tata Kelola Agen AI, Open Knowledge Format v0.2, AI Quote Finder Illustration, CLAUDE.md — Claude Code Guidelines, GEMINI.md — Panduan Gemini & Google Antigravity, knowledge/erd.md — Entity Relationship Diagram, knowledge/frd.md — Functional Requirements Document, knowledge/index.md — Master Knowledge Catalog (+3 more)

### Community 4 - "Infrastructure Configuration"
Cohesion: 0.31
Nodes (9): AppError, Client, AppConfig, init_db_pool(), init_redis_client(), DatabaseConnection, Result, Self (+1 more)

### Community 5 - "Architecture Guidelines"
Cohesion: 0.22
Nodes (10): OKF & Persistent Memory Guidelines, Dual-Mode Embedding (768-Dim), Monorepo Polyglot Architecture, Postgres for Everything, Redis Streams Broker, 6 Engineering Pillars, DISTRIBUTED.md — Arsitektur Pemrosesan Terdistribusi, Docker Infrastructure Configuration (+2 more)

### Community 6 - "Database Migrations"
Cohesion: 0.76
Nodes (6): init_table(), migrate_down(), migrate_status(), migrate_up(), psql_cmd(), migrate.sh script

### Community 7 - "API Response Wrapper"
Cohesion: 0.40
Nodes (4): ApiResponse<T>, Self, T, Value

### Community 8 - "Project Modules"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 10 - "Authentication Security"
Cohesion: 0.67
Nodes (3): POST /api/auth/refresh, POST /api/auth/verify-otp, Security Architecture

### Community 11 - "User Progress Flow"
Cohesion: 0.67
Nodes (3): GET /api/books/search, POST /api/progress/merge, User Journey Flowchart

## Knowledge Gaps
- **53 isolated node(s):** `web`, `POST /api/auth/refresh`, `POST /api/auth/verify-otp`, `GET /api/books/search`, `POST /api/progress/merge` (+48 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **41 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Project Baca — Log Keputusan Arsitektur & Memori Sistem` connect `Architecture Guidelines` to `Project Documentation`?**
  _High betweenness centrality (0.010) - this node is a cross-community bridge._
- **Why does `AGENTS.md — Panduan & Tata Kelola Agen AI` connect `Project Documentation` to `Architecture Guidelines`?**
  _High betweenness centrality (0.009) - this node is a cross-community bridge._
- **Why does `main()` connect `Server Entry Point` to `Infrastructure Configuration`?**
  _High betweenness centrality (0.009) - this node is a cross-community bridge._
- **What connects `web`, `POST /api/auth/refresh`, `POST /api/auth/verify-otp` to the rest of the system?**
  _53 weakly-connected nodes found - possible documentation gaps or missing edges._