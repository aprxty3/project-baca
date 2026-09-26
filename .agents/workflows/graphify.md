---
name: graphify
description: Run the Graphify pipeline to extract and update the repository knowledge graph
---

# Workflow: graphify

Run the Graphify knowledge graph pipeline:

1. Extraction and clustering:
   `graphify update .` or `graphify extract . --backend gemini`
2. Re-cluster graph topology:
   `graphify cluster-only`
3. Export to Obsidian Vault:
   Update notes in `obsidian-vault/` and synchronize `00_DASHBOARD.md`.
