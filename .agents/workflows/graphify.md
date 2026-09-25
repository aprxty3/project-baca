---
name: graphify
description: Menjalankan pipeline Graphify untuk mengekstraksi dan memperbarui graf pengetahuan repositori
---

# Workflow: graphify

Ikuti protokol graphify untuk menjalankan pipeline graf pengetahuan:

1. Ekstraksi dan klasterisasi:
   `graphify extract . --backend gemini`
2. Klasterisasi ulang semantik:
   `graphify cluster-only`
3. Ekspor ke Obsidian Vault:
   Perbarui direktori `/home/aprxty3/ObsidianVaults/project-baca/` dan sinkronkan `00_DASHBOARD.md`.
