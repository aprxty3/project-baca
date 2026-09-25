# Changelog — Project Baca

Seluruh perubahan penting pada proyek ini dicatat secara kronologis dalam berkas ini.
Format changelog ini mengacu pada panduan [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan mematuhi [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] — 2026-09-25

### Ditambahkan (Added)
- **Open Knowledge Format (OKF v0.2):** Implementasi penuh SSOT spesifikasi di direktori `knowledge/` (`prd.md`, `erd.md`, `ux-flow.md`, `frd.md`, `srs.md`, `index.md`, `log.md`).
- **Peta Pengetahuan Graf Graphify:** Ekstraksi 20 simpul dan 15 klaster komunitas di `graphify-out/` dengan visualisasi web interaktif `graph.html`.
- **Ekspor Obsidian Vault:** Pembuatan vault terstruktur di `/home/aprxty3/ObsidianVaults/project-baca/` dengan symlink lokal `obsidian-vault/`, dilengkapi catatan komunitas dan `00_DASHBOARD.md`.
- **Aturan Agen Workspace:** Berkas aturan [.agents/rules/okf_memory.md](.agents/rules/okf_memory.md), [.agents/rules/graphify.md](.agents/rules/graphify.md), dan workflow [.agents/workflows/graphify.md](.agents/workflows/graphify.md).
- **Panduan Kolaborasi Multi-Agen:** Penambahan berkas [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md), [MEMORY.md](MEMORY.md), [GEMINI.md](GEMINI.md), [GUIDE.md](GUIDE.md), [DISTRIBUTED.md](DISTRIBUTED.md), [NOTICE.md](NOTICE.md), dan [.gitignore](.gitignore).
- **Spesifikasi Teknis Lengkap:** Kontrak terinci untuk 23 endpoint REST API dan arsitektur keamanan multi-lapis di `knowledge/srs.md`.
- **Matriks Pengindeksan ERD:** 16 indeks database PostgreSQL 17 termasuk partial index dan vektor HNSW di `knowledge/erd.md`.
- **Arsitektur Navigasi & UX Flow:** 5 wireframe ASCII dan alur rekonsiliasi data pembaca tamu ke akun cloud di `knowledge/ux-flow.md`.
- **Kebutuhan Fungsional & RTM:** Spesifikasi rinci 7 modul sistem di `knowledge/frd.md`.

### Diubah (Changed)
- **Mesin Embedding Vektor:** Mengeliminasi Triton Inference Server dari tumpukan MVP dan mengadopsi arsitektur Dual-Mode Embedding Provider (Google GenAI Gemini API untuk cloud dan FastEmbed CPU ONNX untuk lokal ARM64/x86), serta menstandardisasi dimensi vektor pada 768 dimensi guna memangkas konsumsi RAM indeks HNSW PostgreSQL hingga 50%.
- **Strategi Mesin Pencarian:** Mengganti rencana evaluasi Elasticsearch dengan PostgreSQL 17 Full-Text Search + `pg_trgm` GIN index (<3ms) untuk menghemat alokasi RAM dan mencegah kompleksitas *dual-write*.
- **Message Broker:** Mengadopsi Redis Streams sebagai antrean asinkron tunggal untuk ingestion EPUB dan email transaksional, menolak broker eksternal (RedPanda/RabbitMQ) demi kepatuhan prinsip KISS dan YAGNI.
- **Kebijakan Gaya Dokumentasi:** Menghapus seluruh simbol emotikon dan emoji pada seluruh berkas dokumentasi (*Zero Emoji Policy*).
- **Desain Visual:** Menyelaraskan seluruh spesifikasi antarmuka dengan tema *Vintage Literary / Mid-Century Writer (1900–1950)* dan sakelar bahasa UI dwibahasa `[ID|EN]`.

### Keamanan (Security)
- Perancangan pertahanan berlapis (*Defense-in-Depth*): Proteksi DDoS/WAF tepi Cloudflare, Redis sliding-window rate limiters, perlindungan CSRF SameSite, hashing kata sandi Argon2id, dan OTP 6-digit dengan batas percobaan dan hash SHA-256.
- - Isolasi Berkas Repositori: Memperbarui .gitignore untuk mengabaikan direktori spesifikasi privat (knowledge/ dan obsidian-vault/) serta menerbitkan .env.example untuk mencegah kebocoran kredensial dan path lokal.

---

## [0.1.0] — 2026-09-25

### Ditambahkan (Added)
- **Inisialisasi Proyek:** Pembentukan cetak biru monorepo polyglot [ARCHITECTURE.md](ARCHITECTURE.md) dan ringkasan [README.md](README.md).
- **Infrastruktur Dev Lokal:** Berkas [docker-compose.yml](docker-compose.yml) untuk PostgreSQL 17 + pgvector (port 5433), Redis 7 (port 6380), MinIO S3 (port 9000/9001), dan Mailpit (port 1025/8025).
- **Riset Legalitas Hak Cipta:** Analisis perlindungan ciptaan seumur hidup pencipta + 70 tahun berdasarkan UU No. 28/2014 dan standar karya domain publik Project Gutenberg & Standard Ebooks.
- **Katalog Aset Desain:** Katalog 5 ilustrasi etsa pena klasik Victoria/Edwardian di `assets/README.md`.
