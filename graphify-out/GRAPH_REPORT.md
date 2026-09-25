# Graph Report - project-baca  (2026-09-25)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 51 nodes · 22 edges · 32 communities (4 shown, 28 thin omitted)
- Extraction: 86% EXTRACTED · 14% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.88)
- Token cost: 659 input · 318 output

## Graph Freshness
- Built from commit: `a922834e`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Project Documentation & Requirements
- AI Governance & System Logs
- Authentication & Security
- User Journey & Search
- System Architecture Design
- UX Flow & Navigation
- Distributed Infrastructure
- Offline Content Access
- Performance & Quote Search
- Reader Interface Design
- Library Catalog Design
- Ingestion Monitoring Design
- Project Changelog
- Developer Operations Guide
- Activity Heartbeat Monitoring
- Ingestion Job Tracking
- Admin Book Upload
- API Error Standards
- Atomic Content Retrieval
- User Login
- User Logout
- User Registration
- Chapter Content Retrieval
- Book Detail Retrieval
- Book Catalog Listing
- Chapter Recap Service
- User Account Deletion
- User Profile Retrieval
- Reading Progress Updates
- UX Design Philosophy
- Legal & Licensing
- Project Overview

## God Nodes (most connected - your core abstractions)
1. `knowledge/index.md — Master Knowledge Catalog` - 7 edges
2. `knowledge/frd.md — Functional Requirements Document` - 3 edges
3. `knowledge/prd.md — Product Requirements Document` - 3 edges
4. `AGENTS.md — Panduan & Tata Kelola Agen AI` - 3 edges
5. `knowledge/erd.md — Entity Relationship Diagram` - 2 edges
6. `MEMORY.md — Log Keputusan Arsitektur & Memori Sistem` - 2 edges
7. `Security Architecture` - 2 edges
8. `User Journey Flowchart` - 2 edges
9. `AI Quote Finder Illustration` - 1 edges
10. `GBrain Design Reference` - 1 edges

## Surprising Connections (you probably didn't know these)
- `AI Quote Finder Illustration` --conceptually_related_to--> `knowledge/prd.md — Product Requirements Document`  [EXTRACTED]
  assets/illustrations/retro-rocket-discovery.png → knowledge/prd.md
- `GBrain Design Reference` --conceptually_related_to--> `ARCHITECTURE.md — Cetak Biru Arsitektur & Spesifikasi Desain`  [EXTRACTED]
  assets/references/gbrain-design-reference.png → ARCHITECTURE.md
- `Admin Sorting Illustration` --conceptually_related_to--> `knowledge/ux-flow.md — UX Flow & Navigation`  [EXTRACTED]
  assets/illustrations/admin-sorting-pigeonholes.png → knowledge/ux-flow.md
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
- **Core Reading Experience** — knowledge_srs_books_chapter, knowledge_srs_progress_sync, knowledge_srs_chapter_recap, knowledge_ux_flow_philosophy [INFERRED 0.90]

## Communities (32 total, 28 thin omitted)

### Community 0 - "Project Documentation & Requirements"
Cohesion: 0.32
Nodes (8): AI Quote Finder Illustration, CLAUDE.md — Claude Code Guidelines, GEMINI.md — Panduan Gemini & Google Antigravity, knowledge/erd.md — Entity Relationship Diagram, knowledge/frd.md — Functional Requirements Document, knowledge/index.md — Master Knowledge Catalog, knowledge/prd.md — Product Requirements Document, knowledge/srs.md — Software Requirements Specification

### Community 1 - "AI Governance & System Logs"
Cohesion: 0.50
Nodes (4): AGENTS.md — Panduan & Tata Kelola Agen AI, knowledge/log.md — Audit Trail, MEMORY.md — Log Keputusan Arsitektur & Memori Sistem, PROJECT_LOG.md — Log & Memory Proyek

### Community 2 - "Authentication & Security"
Cohesion: 0.67
Nodes (3): POST /api/auth/refresh, POST /api/auth/verify-otp, Security Architecture

### Community 3 - "User Journey & Search"
Cohesion: 0.67
Nodes (3): GET /api/books/search, POST /api/progress/merge, User Journey Flowchart

## Knowledge Gaps
- **42 isolated node(s):** `AI Quote Finder Illustration`, `Library Catalog Illustration`, `Ingestion Monitor Illustration`, `GBrain Design Reference`, `Admin Sorting Illustration` (+37 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **28 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `knowledge/index.md — Master Knowledge Catalog` connect `Project Documentation & Requirements` to `AI Governance & System Logs`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Why does `AGENTS.md — Panduan & Tata Kelola Agen AI` connect `AI Governance & System Logs` to `Project Documentation & Requirements`?**
  _High betweenness centrality (0.021) - this node is a cross-community bridge._
- **What connects `AI Quote Finder Illustration`, `Library Catalog Illustration`, `Ingestion Monitor Illustration` to the rest of the system?**
  _42 weakly-connected nodes found - possible documentation gaps or missing edges._