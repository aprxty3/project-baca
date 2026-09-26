---
trigger: always_on
description: Graphify knowledge graph consultation guidelines in graphify-out/ for architecture and module relationships.
---

## Graphify Knowledge Graph Protocol

This repository contains a Graphify knowledge graph in `graphify-out/` and an Obsidian Vault in `obsidian-vault/`.

### Navigation Rules:
1. **Focused Architectural Query:** Use `graphify query "<question>"`, `shortest_path`, or `get_node` instead of scanning large raw files.
2. **High-Level Exploration:** Consult [graphify-out/GRAPH_REPORT.md](graphify-out/GRAPH_REPORT.md) for architecture overviews, god nodes, and community clusters.
3. **Post-Change Synchronization:** After significant modifications to code or specifications, run `graphify update .` or `graphify cluster-only` to maintain graph topology.
