# Project Baca: Cetak Biru Arsitektur & Spesifikasi Desain

Dokumen ini merangkum arsitektur teknis, pemilihan teknologi polyglot, struktur monorepo, pola Domain-Driven Design (DDD) ramah Rust, pipeline AI/NLP, konfigurasi infrastruktur Docker, serta metodologi pengembangan untuk **Project Baca**.

---

## 1. Visi Produk & Pengalaman Membaca

Project Baca adalah platform e-reader modern yang memadukan dua keunggulan:
1. **Pengalaman Membaca Mendalam (Kindle & Apple Books Style):** Tipografi bersih, naskah reflowable (menyesuaikan layar ponsel), paginasi berbasis ketukan layar (*tap-to-turn*), bebas iklan invasif, dan *offline-first* via IndexedDB.
2. **Kecerdasan Ringkasan & Kutipan (Blinkist & Deepstash Style):** Rangkuman kartu wawasan atomik (*Atomic Insight Cards*) per bab, tombol *Rekap Bab Sebelumnya* (anti-spoiler), dan *Scoped Quote Finder* di Halaman Sinopsis Buku (*Book Overview*).
3. **Katalog Legal Penuh:** Berbasis naskah domain publik dan lisensi terbuka (*Standard Ebooks, Project Gutenberg, Let's Read Asia, Wikisumber*).

---

## 2. Struktur Monorepo Polyglot

Proyek dirancang dalam monorepo yang membagi tanggung jawab secara tegas antara performa I/O tinggi (Rust), keluwesan NLP/AI (Python), arsitektur Dual-Mode Embedding (Google GenAI API & FastEmbed CPU), dan antarmuka web (WASM):

```text
project-baca/
├── Cargo.toml                       # Root workspace manifest (Rust)
├── Makefile                         # Otomasi pengembang: dev, db-up, migrate-up, migrate-down
├── docker-compose.yml               # Postgres 17 pgvector, Redis, MinIO, Mailpit
├── README.md                        # Ringkasan proyek & panduan singkat
├── ARCHITECTURE.md                  # Cetak biru arsitektur monorepo polyglot (dokumen ini)
├── PROJECT_LOG.md                   # Catatan sprint & status operasional backlog
├── AGENTS.md                        # Aturan operasional multi-agen AI
├── CLAUDE.md                        # Panduan operasional Claude Code
├── MEMORY.md                        # Log keputusan arsitektural ("the why")
├── GEMINI.md                        # Panduan Antigravity & model Gemini
├── GUIDE.md                         # Panduan instalasi dan pengujian lokal mendalam
├── DISTRIBUTED.md                   # Arsitektur worker ingestion, Redis Streams, dan Dual-Mode Embedding
├── CHANGELOG.md                     # Riwayat rilis berstandar SemVer
├── NOTICE.md                        # Pernyataan hak cipta domain publik (UU 28/2014)
├── .gitignore                       # Aturan pengabaian kompilasi dan cache
│
├── .agents/                         # Aturan persisten AI agent & alur kerja Graphify
│   ├── rules/                       # okf_memory.md, graphify.md
│   └── workflows/                   # graphify.md
│
├── assets/                          # Aset visual & panduan desain
│   ├── README.md
│   ├── illustrations/               # Etsa pena klasik Victoria/Edwardian
│   └── references/                  # Referensi visual gbrain.io
│
├── knowledge/                       # OKF v0.2 Knowledge Vault (Spec SSOT)
│   ├── index.md                     # Master catalog & progressive disclosure
│   ├── prd.md                       # Product Requirements Document
│   ├── erd.md                       # Entity Relationship Diagram & Schema
│   ├── ux-flow.md                   # UX Flow & wireframe layout
│   ├── frd.md                       # Functional Requirements Document
│   ├── srs.md                       # Software Requirements Specification
│   └── log.md                       # Audit trail kronologis
│
├── obsidian-vault/                  # Symlink ke Obsidian Vault di /home/aprxty3/ObsidianVaults/project-baca/
├── graphify-out/                    # Graf pengetahuan Graphify (nodes, edges, komunitas)
│
├── migrations/                      # Riwayat migrasi database terkelola (.up.sql & .down.sql)
│   ├── 20260925000001_init_extensions.up.sql
│   ├── 20260925000001_init_extensions.down.sql
│   ├── 20260925000002_create_users_and_roles.up.sql
│   ├── 20260925000002_create_users_and_roles.down.sql
│   ├── 20260925000003_create_books_and_tags.up.sql
│   ├── 20260925000003_create_books_and_tags.down.sql
│   ├── 20260925000004_create_chapters_and_chunks.up.sql
│   ├── 20260925000004_create_chapters_and_chunks.down.sql
│   └── 20260925000005_create_progress_and_gamification.up.sql
│
├── crates/                          # Rust Cargo Workspace
│   ├── domain/                      # Pure Business Logic & Entities (Book, Reader, Progress)
│   ├── shared/                      # DTOs, i18n Dictionary, & Validasi (Shared Axum & WASM)
│   ├── infra/                       # Database ORM (SeaORM), Redis client, S3/MinIO client
│   ├── server/                      # HTTP API Gateway (Axum REST, Auth JWT, Static PWA)
│   └── web/                         # Frontend Client-Side (Leptos 0.7 WASM PWA)
│       ├── index.html
│       ├── manifest.json
│       ├── service-worker.js
│       └── src/
│           ├── components/          # ReaderView, BookCard, QuoteModal, AtomicCards
│           ├── context/             # LocaleContext (i18n ID ⇄ EN)
│           ├── pages/               # Library, BookOverview, Reader, Admin
│           └── storage/             # IndexedDB wrapper via `rexie`
│
├── services/                        # Layanan Pendukung Non-Rust
│   └── worker/                      # Python AI Ingestion & NLP Worker
│       ├── pyproject.toml / requirements.txt
│       ├── ingestion/               # EPUB parsing, text cleaning, scene chunking
│       └── tasks/                   # Gemini SDK summarization & Dual-Mode Embedding (Gemini / FastEmbed)
│
└── apps/
    └── mobile/                      # Tauri v2 Wrapper (Target: Android / iOS - Fase 2)
```

---

## 3. Desain DDD (Domain-Driven Design) Ramah Rust

* **Domain Murni Tanpa Bloat OOP:** Struct dan Enums murni dengan *Newtype Pattern* (contoh: `BookId(Uuid)`, `ChapterNumber(u32)`).
* **Ports & Adapters (Hexagonal):** Trait sederhana dan idiomatik untuk repository database dan storage:
  ```rust
  pub trait BookRepository: Send + Sync {
      async fn find_by_id(&self, id: &BookId) -> Result<Option<Book>, DomainError>;
      async fn list_books(&self, filter: &BookFilter) -> Result<Vec<BookSummary>, DomainError>;
  }
  ```
* **Shared DTOs (`crates/shared`):** Mendefinisikan struct pertukaran data yang dikompilasi ke native untuk Axum dan dikompilasi ke WebAssembly untuk Leptos.

---

## 4. Pipeline Ingestion & Arsitektur AI Semantik

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       ALUR INGESTION & PIPELINE AI                          │
└─────────────────────────────────────────────────────────────────────────────┘
.
 1. INGESTION (Admin Upload -> Axum -> Redis Streams -> Python Worker)
    [Admin Upload .epub] ──► [Axum API] ──► Simpan raw .epub ke S3/MinIO
                                  │
                                  ▼ (Push Event)
                         [Redis Streams Queue]
                                  │
                                  ▼ (Pop Job)
                         [Python NLP Worker]
                           ├── Parsing struktur EPUB (XHTML per bab)
                           ├── Sanitasi HTML & deteksi scene break
                           ├── Ekstraksi metadata, cover, & hitung word count
                           ├── Chunking teks novel (~300-500 kata)
                           ├── Request embedding (Gemini API / FastEmbed CPU 768-dim)
                           └── Simpan ke PostgreSQL 17 + pgvector (tabel book_chunks)

 -----------------------------------------------------------------------------

 2. PENCARIAN KATALOG BUKU (Discovery Search: PostgreSQL 17 FTS + pg_trgm)
    [User di Beranda: Ketik "Sherlok" atau "Petualangan"]
            │
            ▼
    [Axum Endpoint: GET /api/books?q=sherlok&lang=id]
            │
            ▼
    [PostgreSQL 17 FTS + Trigram Query (GIN Index)]:
      SELECT id, title, author, language, primary_theme, cover_url,
             similarity(title || ' ' || author, $1) AS score
      FROM books
      WHERE (title % $1 OR author % $1 OR to_tsvector('simple', title || ' ' || description) @@ plainto_tsquery($1))
        AND status = 'published'
      ORDER BY score DESC, title ASC
      LIMIT 20;
            │
            ▼
    [Hasil Instan < 3ms] ──► Menampilkan Kartu Buku (Toleran Saltik / Typo-Tolerant)
    * Catatan: Tidak memerlukan Elasticsearch (menghemat 2-4 GB RAM JVM & bebas latensi sinkronisasi).

 -----------------------------------------------------------------------------

 3. SCOPED QUOTE FINDER (Halaman Detail Buku: pgvector HNSW)
    [User di Book Overview: "Cari kutipan pengorbanan cinta"]
            │
            ▼
    [Axum Endpoint: POST /api/books/{id}/quotes/search]
            │
            ▼
    [Query Embedding via Gemini API / FastEmbed CPU (768-dim)]
            │
            ▼
    [PostgreSQL pgvector Query]:
      SELECT chunk_text, chapter_number, cosine_distance
      FROM book_chunks
      WHERE book_id = $1
      ORDER BY embedding <=> $query_vector
      LIMIT 5;
            │
            ▼
    [Hasil Ditampilkan di Modal < 10ms] ──► Opsi: Export ke Gambar Kutipan

 -----------------------------------------------------------------------------

 4. ATOMIC INSIGHT CARDS & CATCH-UP RECAP
    [User Buka Bab Baru atau Klik "Rekap Bab Sebelumnya"]
            │
            ▼
    [Cek Cache tldr_cache di PostgreSQL]
            ├── Jika sudah ada: Return instan (0 token cost!)
            └── Jika belum ada: Python worker generate 3-5 kartu atomik via Gemini
```

---

## 5. Arsitektur Frontend Leptos WASM & PWA

* **Reflowable Paginated Layout:** Membagi naskah bab menjadi kolom layar horizontal menggunakan CSS multi-column. Pembalikan halaman dihitung berdasarkan pergeseran kolom per layar (*horizontal paging*).
* **Anchor-based Positioning:** Posisi baca dikunci menggunakan Anchor Node DOM / CFI karakter offset bab, sehingga saat layar di-rotate (portrait ⇄ landscape) pembaca tidak terlempar ke bab lain.
* **IndexedDB via `rexie`:** Buku yang ditandai offline disimpan dalam bentuk HTML bab bersih dan gambar WebP terkompresi di IndexedDB browser, menjamin operasional penuh saat offline.
* **i18n UI Switcher (`[ ID | EN ]`):** Menggunakan `LocaleContext` reaktif di Leptos. Seluruh teks antarmuka luar buku berganti seketika tanpa *page reload*, dengan persistensi di `localStorage`.

---

## 6. Identitas Desain Visual (Vintage Literary 1900–1950)

* **Tema Warna:**
  * Mode Gelap Utama (*Dark Roast/Espresso*): Latar `#1F1916`, permukaan kartu `#29211C`, teks gading `#F0EAE1`, tombol aksi tanah liat terakota `#CE734E`.
  * Mode Terang Kertas (*Antique Parchment*): Latar `#F7F4EE`, tinta arang mesin ketik `#231D19`, garis pembatas litografi `#DBD3C5`.
* **Tipografi:** Editorial Serif (*Playfair Display / EB Garamond*) dengan aksen miring (*flowing italics*), dipadukan dengan Typewriter Monospace (*Courier Prime*) untuk label dan metadata.
* **Ilustrasi:** Etsa pena klasik Victoria/Edwardian (*cross-hatching*) tersimpan di `assets/illustrations/`.

---

## 7. Setup Infrastruktur Lokal (`docker-compose.yml`)

1. **PostgreSQL 17 + pgvector (Port 5433):** Database relasional dan indeks HNSW vektor semantik.
2. **Redis 7 (Port 6380):** Manajemen session, rate limiting kueri AI, dan antrean pesan asynchronous (Redis Streams) untuk komunikasi Axum ⇄ Python worker.
3. **MinIO (Port 9000 & Console 9001):** S3-compatible object storage untuk file `.epub` mentah dan aset cover resolusi tinggi.
4. **Mailpit (SMTP Port 1025 & Web UI 8025):** Server pengujian email lokal untuk verifikasi pendaftaran akun dan reset password.

---

## 8. Lapisan Data & Siklus Hidup Migrasi: ORM (SeaORM) & Makefile

Akses basis data di backend Rust Axum (`crates/infra`) menggunakan **SeaORM** (ORM asinkron berbasis Tokio/SQLx):
* **Keunggulan SeaORM:** Pemodelan entitas yang *type-safe*, *dynamic query builder*, penanganan relasi (1-to-many, many-to-many), serta kompatibilitas langsung dengan kueri kustom PostgreSQL (`pgvector` dan `pg_trgm`).
* **Siklus Hidup Migrasi SQL Terkelola (`migrations/`):**
  Perubahan skema database dikelola melalui pasangan file SQL murni berekstensi `.up.sql` dan `.down.sql`:
  * `<timestamp>_<nama_migrasi>.up.sql`: Berisi perintah DDL untuk membuat atau memodifikasi tabel, indeks, dan ekstensi.
  * `<timestamp>_<nama_migrasi>.down.sql`: Berisi kebalikan perintah DDL (*rollback*) untuk membatalkan perubahan secara bersih.
* **Otomasi Pengembang (`Makefile`):**
  Seluruh alur kerja pengembangan diorkestrasi melalui perintah makefile standar:
  * `make dev`: Menjalankan seluruh kontainer pendukung Docker dan dev server secara simultan.
  * `make db-up`: Memulai kontainer Postgres 17, Redis, MinIO, dan Mailpit.
  * `make db-down`: Menghentikan kontainer.
  * `make migrate-up`: Menjalankan seluruh migrasi yang belum diaplikasikan.
  * `make migrate-down`: Membatalkan (*rollback*) 1 langkah migrasi terakhir.
  * `make migrate-reset`: Menghapus skema dan menjalankan ulang seluruh migrasi dari awal.

---

## 9. Arsitektur Kecerdasan Pengembangan (Quad-Layer System One)

Dalam proses rekayasa perangkat lunak, tim pengembang mengorkestrasi kolaborasi multi-agent (**Google Antigravity** sebagai Architect/PM dan **Claude Code** sebagai Lead Engineer) menggunakan metodologi Quad-Layer:
* **Layer 0 (Jev / Laya):** *Reflex gate* (<0.2 ms) untuk klasifikasi intent perintah dan *guardrail* keamanan eksekusi perintah terminal shell.
* **Layer 1 (Graphify):** Graph AST kode monorepo untuk navigasi simbol dan hierarki fungsi bebas halusinasi.
* **Layer 2 (OKF Vault v0.2):** Vault `knowledge/` sebagai sumber kebenaran spesifikasi dengan frontmatter terverifikasi dan *progressive disclosure*.
* **Layer 3 (GBrain):** Basis data Postgres pgvector untuk memori keputusan lintas sesi kerja agen.

---

## 10. Prinsip Rekayasa Perangkat Lunak Inti (Core Engineering Invariants)

Seluruh komponen dalam monorepo Project Baca wajib mematuhi standar rekayasa berikut:

1. **ROBUST:**
   * Tidak ada `unwrap()` / `expect()` pada kode Rust di lingkungan produksi. Semua galat dimodelkan secara terstruktur dengan `thiserror` dan dikonversi ke respon HTTP aman tanpa mengekspos jejak internal (*stack trace*).
   * Integritas data dijaga di tingkat basis data menggunakan kendala kunci asing (`ON DELETE CASCADE` / `RESTRICT`) dan kendala `CHECK`.
   * Frontend Leptos WASM menerapkan strategi *graceful degradation* saat offline melalui pembacaan data cadangan di `IndexedDB` (`rexie`).
2. **SCALABLE:**
   * Gateway Axum bersifat *stateless* sehingga dapat direplikasi secara horizontal.
   * Pencarian semantik kutipan diisolasi per buku (`WHERE book_id = $1`) pada indeks HNSW pgvector.
   * Pencarian katalog buku menggunakan indeks parsial GIN yang hanya menyaring baris aktif `WHERE status = 'published'`.
3. **EASY TO MAINTAIN:**
   * Batasan monorepo antar crate terisolasi secara modular (`domain`, `shared`, `infra`, `server`, `web`).
   * Perubahan skema basis data selalu memiliki skrip migrasi berpasangan `.up.sql` dan `.down.sql` yang reversibel dan deterministik.
   * Otomasi pengembang distandardisasi melalui `Makefile`.
4. **DRY (Don't Repeat Yourself):**
   * Struct data transfer (DTO) dan aturan validasi didefinisikan satu kali pada `crates/shared` untuk digunakan bersama oleh Axum dan Leptos WASM.
   * Kamus i18n dwibahasa (ID/EN) dikelola secara terpusat dalam konteks reaktif.
5. **KISS (Keep It Simple, Stupid):**
   * Mengadopsi prinsip *"Postgres for Everything"* (relasional, FTS leksikal + Trigram, dan pgvector semantik) dalam satu mesin tanpa membebani sistem dengan klaster Elasticsearch terpisah.
   * Menggunakan Redis Streams untuk antrean pekerjaan asinkron daripada broker terdistribusi berat (RedPanda/RabbitMQ).
   * Pemodelan DDD pragmatis yang ramah terhadap *borrow checker* Rust tanpa kerumitan hierarki OOP.
6. **YAGNI (You Aren't Gonna Need It):**
   * Fokus mutlak pada kebutuhan MVP: membaca buku naskah domain publik dengan tipografi nyaman, ringkasan atomik Deepstash, pencarian kutipan cepat, dan mode offline. Menolak implementasi prematur fitur-fitur kompleks yang baru dijadwalkan pada Fase 2.

