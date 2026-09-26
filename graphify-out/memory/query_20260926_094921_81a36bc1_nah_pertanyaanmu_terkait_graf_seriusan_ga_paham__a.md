---
type: "query"
date: "2026-09-26T09:49:21.993052+00:00"
question: "nah pertanyaanmu terkait graf seriusan ga paham, apalagi across architetureal boundaries gitu. tolong deh telusuri jalur data dan dependencynya."
contributor: "graphify"
outcome: "useful"
source_nodes: ["AppState", "AppConfig", "TestHarness", "get_book"]
---

# Q: nah pertanyaanmu terkait graf seriusan ga paham, apalagi across architetureal boundaries gitu. tolong deh telusuri jalur data dan dependencynya.

## Answer

AppState connects AppConfig (infra), Router (server), get_book (routes), and TestHarness (tests) as the central Axum dependency injection bridge across crate and layer boundaries.

## Outcome

- Signal: useful

## Source Nodes

- AppState
- AppConfig
- TestHarness
- get_book