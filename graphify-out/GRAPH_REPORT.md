# Graph Report - project-baca  (2026-09-25)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 20 nodes · 5 edges · 15 communities (1 shown, 14 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Product Requirements and Diagrams
- System Architecture Design
- Admin UX Navigation
- Reader Interface Design
- Library Catalog UI
- Data Ingestion Monitoring
- Infrastructure Configuration
- Knowledge Bundle Index
- Software Requirements Specification
- SMTP Email Server
- Object Storage Service
- Vector Database Storage
- Project Logs and Memory
- Project Overview
- Caching and Streaming

## God Nodes (most connected - your core abstractions)
1. `Functional Requirements Document` - 2 edges
2. `Product Requirements Document` - 2 edges
3. `Entity Relationship Diagram` - 1 edges
4. `AI Quote Finder Illustration` - 1 edges
5. `Architecture Blueprint` - 1 edges
6. `GBrain Design Reference` - 1 edges
7. `UX Flow & Navigation` - 1 edges
8. `Admin Sorting Illustration` - 1 edges
9. `MinIO Object Storage` - 0 edges
10. `PostgreSQL 17 + pgvector` - 0 edges

## Surprising Connections (you probably didn't know these)
- `AI Quote Finder Illustration` --conceptually_related_to--> `Product Requirements Document`  [EXTRACTED]
  assets/illustrations/retro-rocket-discovery.png → knowledge/prd.md
- `GBrain Design Reference` --conceptually_related_to--> `Architecture Blueprint`  [EXTRACTED]
  assets/references/gbrain-design-reference.png → ARCHITECTURE.md
- `Admin Sorting Illustration` --conceptually_related_to--> `UX Flow & Navigation`  [EXTRACTED]
  assets/illustrations/admin-sorting-pigeonholes.png → knowledge/ux-flow.md
- `Functional Requirements Document` --cites--> `Entity Relationship Diagram`  [EXTRACTED]
  knowledge/frd.md → knowledge/erd.md
- `Functional Requirements Document` --cites--> `Product Requirements Document`  [EXTRACTED]
  knowledge/frd.md → knowledge/prd.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Local Infrastructure Stack** — postgres_db, redis_cache, minio_s3, mailpit_smtp [EXTRACTED 1.00]
- **OKF v0.2 Knowledge Vault** — knowledge_prd_md, knowledge_erd_md, knowledge_frd_md, knowledge_srs_md, knowledge_ux_flow_md [EXTRACTED 1.00]

## Communities (15 total, 14 thin omitted)

### Community 0 - "Product Requirements and Diagrams"
Cohesion: 0.50
Nodes (4): AI Quote Finder Illustration, Entity Relationship Diagram, Functional Requirements Document, Product Requirements Document

## Knowledge Gaps
- **18 isolated node(s):** `Entity Relationship Diagram`, `AI Quote Finder Illustration`, `Architecture Blueprint`, `GBrain Design Reference`, `MinIO Object Storage` (+13 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **14 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What connects `Entity Relationship Diagram`, `AI Quote Finder Illustration`, `Architecture Blueprint` to the rest of the system?**
  _18 weakly-connected nodes found - possible documentation gaps or missing edges._