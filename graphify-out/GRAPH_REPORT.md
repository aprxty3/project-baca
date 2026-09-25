# Graph Report - project-baca  (2026-09-26)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 142 nodes · 177 edges · 41 communities (11 shown, 30 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 6 edges (avg confidence: 0.88)
- Token cost: 949 input · 413 output

## Graph Freshness
- Built from commit: `8991aa49`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core Domain Models
- API Data Transfer Objects
- Server Entry and State
- Architecture and Project Governance
- Infrastructure and Configuration
- Project Documentation and Requirements
- Generic API Response Wrapper
- Project Module Structure
- Web Frontend Components
- Authentication Security Flow
- Book Search and Progress
- Knowledge Graph Protocol
- System Design Blueprints
- UX and Navigation Design
- Distributed Infrastructure
- Offline Mode Support
- Performance and Search Metrics
- Reader Interface Design
- Library Catalog Design
- Data Ingestion Monitoring
- Project Changelog
- Developer Operations Guide
- Activity Heartbeat API
- Ingestion Job Management
- Book Upload API
- API Error Standards
- Atomic Cards API
- User Login API
- User Logout API
- User Signup API
- Chapter Content API
- Book Detail API
- Book Listing API
- Chapter Recap API
- Delete Account API
- User Profile API
- Update Reading Progress
- UX Philosophy
- Legal and Licensing
- Project Overview

## God Nodes (most connected - your core abstractions)
1. `Book` - 7 edges
2. `main()` - 7 edges
3. `knowledge/index.md — Master Knowledge Catalog` - 7 edges
4. `MEMORY.md — Log Keputusan Arsitektur & Memori Sistem` - 7 edges
5. `User` - 6 edges
6. `BookDetailDto` - 6 edges
7. `AppConfig` - 6 edges
8. `init_db_pool()` - 6 edges
9. `init_redis_client()` - 6 edges
10. `AppError` - 6 edges

## Surprising Connections (you probably didn't know these)
- `GBrain Design Reference` --conceptually_related_to--> `ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain`  [EXTRACTED]
  assets/references/gbrain-design-reference.png → ARCHITECTURE.md
- `Admin Sorting Illustration` --conceptually_related_to--> `knowledge/ux-flow.md — UX Flow & Navigation`  [EXTRACTED]
  assets/illustrations/admin-sorting-pigeonholes.png → knowledge/ux-flow.md
- `AI Quote Finder Illustration` --conceptually_related_to--> `knowledge/prd.md — Product Requirements Document`  [EXTRACTED]
  assets/illustrations/retro-rocket-discovery.png → knowledge/prd.md
- `AGENTS.md — Panduan & Tata Kelola Agen AI` --references--> `knowledge/index.md — Master Knowledge Catalog`  [EXTRACTED]
  AGENTS.md → knowledge/index.md
- `CLAUDE.md — Claude Code Guidelines` --references--> `knowledge/index.md — Master Knowledge Catalog`  [EXTRACTED]
  CLAUDE.md → knowledge/index.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Authentication & Account Management Flow** — knowledge_srs_auth_signup, knowledge_srs_auth_verify_otp, knowledge_srs_auth_login, knowledge_srs_auth_refresh, knowledge_srs_auth_logout [EXTRACTED 1.00]
- **Knowledge Governance Stack** — agents_rules_okf_memory, agents_rules_graphify, agents_workflows_graphify, knowledge_log_md [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]
- **OKF v0.2 Specification Suite** — knowledge_index_md, knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md, knowledge_log_md [EXTRACTED 1.00]
- **Semantic Intelligence Cluster** — knowledge_srs_quotes_search, knowledge_srs_atomic_cards, knowledge_srs_chapter_recap [EXTRACTED 1.00]
- **Architectural Guardrails** — engineering_pillars, memory_md, agents_rules_okf_memory [INFERRED 0.85]
- **Core Reading Experience** — knowledge_srs_books_chapter, knowledge_srs_progress_sync, knowledge_srs_chapter_recap, knowledge_ux_flow_philosophy [INFERRED 0.90]

## Communities (41 total, 30 thin omitted)

### Community 0 - "Core Domain Models"
Cohesion: 0.23
Nodes (17): Book, BookChunk, BookStatus, Chapter, DomainError, ReadingProgress, ReadingStreak, DateTime (+9 more)

### Community 1 - "API Data Transfer Objects"
Cohesion: 0.23
Nodes (19): ApiResponse, BookDetailDto, BookSummaryDto, ChapterSummaryDto, ErrorPayload, LoginRequest, QuoteSearchRequest, QuoteSearchResultDto (+11 more)

### Community 2 - "Server Entry and State"
Cohesion: 0.20
Nodes (13): Arc, Box, api_health_check(), AppState, health_check(), main(), Client, DatabaseConnection (+5 more)

### Community 3 - "Architecture and Project Governance"
Cohesion: 0.18
Nodes (12): AGENTS.md — Panduan & Tata Kelola Agen AI, OKF & Persistent Memory Guidelines, Asset Catalog, Web Entry Point, Dual-Mode Embedding Decision, Cargo Workspace Monorepo, Postgres for Everything Decision, Vintage Literary UI Design (+4 more)

### Community 4 - "Infrastructure and Configuration"
Cohesion: 0.31
Nodes (9): AppConfig, init_db_pool(), init_redis_client(), Client, DatabaseConnection, Result, Self, String (+1 more)

### Community 5 - "Project Documentation and Requirements"
Cohesion: 0.32
Nodes (8): AI Quote Finder Illustration, CLAUDE.md — Claude Code Guidelines, GEMINI.md — Panduan Gemini & Google Antigravity, knowledge/erd.md — Entity Relationship Diagram, knowledge/frd.md — Functional Requirements Document, knowledge/index.md — Master Knowledge Catalog, knowledge/prd.md — Product Requirements Document, knowledge/srs.md — Software Requirements Specification

### Community 6 - "Generic API Response Wrapper"
Cohesion: 0.40
Nodes (4): ApiResponse<T>, Self, T, Value

### Community 7 - "Project Module Structure"
Cohesion: 0.70
Nodes (5): domain, infra, server, shared, web

### Community 9 - "Authentication Security Flow"
Cohesion: 0.67
Nodes (3): POST /api/auth/refresh, POST /api/auth/verify-otp, Security Architecture

### Community 10 - "Book Search and Progress"
Cohesion: 0.67
Nodes (3): GET /api/books/search, POST /api/progress/merge, User Journey Flowchart

## Knowledge Gaps
- **48 isolated node(s):** `web`, `GET /api/books/search`, `POST /api/progress/merge`, `ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain`, `GBrain Design Reference` (+43 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **30 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppError` connect `Infrastructure and Configuration` to `API Data Transfer Objects`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **Why does `main()` connect `Server Entry and State` to `Infrastructure and Configuration`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Why does `init_db_pool()` connect `Infrastructure and Configuration` to `Server Entry and State`?**
  _High betweenness centrality (0.025) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `main()` (e.g. with `init_db_pool()` and `init_redis_client()`) actually correct?**
  _`main()` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `web`, `GET /api/books/search`, `POST /api/progress/merge` to the rest of the system?**
  _48 weakly-connected nodes found - possible documentation gaps or missing edges._