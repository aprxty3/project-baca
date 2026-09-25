---
trigger: always_on
description: Panduan konsultasi graf pengetahuan Graphify di graphify-out/ untuk pertanyaan arsitektur dan relasi modul.
---

## Graphify Knowledge Graph Protocol

Repositori ini memiliki graf pengetahuan Graphify di direktori `graphify-out/` dan Obsidian Vault di `obsidian-vault/`.

### Aturan Navigasi:
1. **Pemeriksaan Arsitektur Terfokus:** Ketika menelusuri arsitektur atau keterkaitan berkas, gunakan kueri terfokus melalui CLI `graphify query "<pertanyaan>"`, `shortest_path`, atau `get_node` daripada memindai teks mentah berukuran besar.
2. **Eksplorasi Tingkat Tinggi:** Baca [graphify-out/GRAPH_REPORT.md](graphify-out/GRAPH_REPORT.md) untuk tinjauan umum arsitektur, daftar god nodes, dan sebaran klaster komunitas.
3. **Sinkronisasi Pasca Perubahan:** Setelah melakukan penambahan atau modifikasi berkas kode/spesifikasi yang signifikan pada sesi kerja, jalankan perintah `graphify cluster-only` atau `graphify update .` untuk memperbarui topologi graf.
