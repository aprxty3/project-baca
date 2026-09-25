---
type: Technical Architecture Specification
title: "Project Baca — Distributed Worker, Ingestion & Scaling Architecture"
description: "Arsitektur pemrosesan terdistribusi untuk pipeline ingestion EPUB, antrean Redis Streams, Dual-Mode Embedding (Gemini API & FastEmbed CPU), dan skalabilitas horizontal."
tags: [distributed, worker, ingestion, redis-streams, embedding, gemini, fastembed, scalability, okf]
---

# DISTRIBUTED.md — Arsitektur Pemrosesan Terdistribusi & Pipeline Ingestion

Dokumen ini mendefinisikan arsitektur pemrosesan terdistribusi (*distributed compute*), manajemen antrean asinkron, dan strategi skalabilitas horizontal untuk **Project Baca**.

---

## 1. Topologi Pemrosesan Terdistribusi

Sistem dirancang dengan pemisahan tegas antara lapisan layanan HTTP interaktif yang *stateless* dan lapisan pemrosesan intensif di latar belakang (*background workers*):

```text
+-------------------+       +-------------------+       +-----------------------+
|  Klien Web/PWA    | ----> |  Axum API Server  | ----> |  Redis 7 Streams      |
|  (Leptos WASM)    | <---- |  (Stateless Node) |       |  Stream: epub:ingest  |
+-------------------+       +-------------------+       +-----------------------+
                                                                    |
                                                                    v
+-----------------------+       +-------------------+       +-----------------------+
|  Dual-Mode Embedding  | <==== |  Python Ingestion | <==== |  Consumer Group:      |
|  - Gemini API (Cloud) |       |  Worker Instances |       |  ingestion-workers    |
|  - FastEmbed CPU (ARM)| ====> |  (Horizontal Pods)|       |  - Auto-claim pending |
+-----------------------+       +-------------------+       +-----------------------+
                                          |
                                          v
                                +-------------------+
                                |  PostgreSQL 17    |
                                |  + pgvector HNSW  |
                                |  + MinIO / S3 R2  |
                                +-------------------+
```

---

## 2. Pipeline Ingestion EPUB Asinkron

Alur kerja ingestion naskah buku ranah publik berjalan melalui 5 tahapan terisolasi:

1. **Pengunggahan & Persistensi Objek:**
   - Admin mengunggah berkas `.epub` melalui endpoint `/api/admin/books/upload`.
   - Axum memverifikasi magic bytes berkas dan mengunggah berkas mentah ke MinIO/S3 bucket `baca-epubs/raw/<book_id>.epub`.
   - Axum mempublikasikan pesan tugas ke Redis Stream `stream:epub:ingestion`.
2. **Konsumsi & Parsing Terisolasi (Python Worker):**
   - Worker mengambil tugas dari consumer group `ingestion-workers` via perintah `XREADGROUP`.
   - Memvalidasi metadata OPF, mengekstrak cover ke bucket `baca-covers/`, dan melakukan sanitasi HTML (menghapus script, style, iklan bawaan).
3. **Scene-based & Semantic Chunking:**
   - Memecah narasi bab menjadi potongan semantik (*chunks*) berukuran 300–500 kata dengan batas scene/paragraf alami (bukan pemotongan teks acak).
   - Menghasilkan DOM CFI (*Canonical Fragment Identifier*) untuk setiap chunk agar dapat diarahkan langsung pada reader.
4. **Vektorisasi Batch via Dual-Mode Embedding Provider (768 Dimensi):**
   - Worker memproses kumpulan chunk menggunakan `EmbeddingProvider` yang dikonfigurasi melalui variabel lingkungan:
     - **Mode Cloud (Default):** Google GenAI Gemini API (`text-embedding-004`). Memproses vektor 768 dimensi di cloud secara instan tanpa membutuhkan kartu grafis GPU dan tanpa beban memori RAM lokal.
     - **Mode Lokal / Offline:** Pustaka `fastembed` berbasis ONNX Runtime yang dioptimalkan untuk CPU (termasuk arsitektur ARM64 / Apple Silicon / Graviton dengan instruksi NEON). Sangat hemat RAM (<150 MB) dan ukuran model hanya ~60 MB.
   - Standarisasi vektor pada **768 dimensi** menghemat alokasi memori RAM indeks HNSW PostgreSQL hingga 50% dibanding model 1536 dimensi.
5. **Penyimpanan Batch & Indeksasi HNSW:**
   - Vektor dan teks chunk disimpan secara massal (*bulk insert*) ke tabel `book_chunks`.
   - Status buku diperbarui menjadi `published` di tabel `books`.

---

## 3. Strategi Skalabilitas Semantik (Scoped Vector Search)

Tantangan utama dari pencarian vektor berbasis graf (*HNSW*) adalah konsumsi memori RAM yang meningkat seiring jumlah node vektor:

* **Pola Scoped Search (`WHERE book_id = $1`):**
  - Berbeda dari sistem RAG umum yang mencari di seluruh korpus, Project Baca membatasi pencarian kutipan hanya di dalam buku yang sedang dibaca pengguna.
  - Kueri SQL:
    ```sql
    SELECT id, chapter_id, content, cfi_range, 1 - (embedding <=> $2) AS similarity
    FROM book_chunks
    WHERE book_id = $1
    ORDER BY embedding <=> $2
    LIMIT 5;
    ```
  - Isolasi ini memungkinkan ratusan ribu chunk buku tersimpan di disk tanpa membebani indeks HNSW memori global secara berlebih.

---

## 4. Keandalan Antrean & Toleransi Kesalahan (Fault Tolerance)

1. **Pengakuan Pesan (*Message Acknowledgment*):**
   - Setiap tugas hanya dihapus dari antrean pending setelah worker memanggil `XACK`.
2. **Pemulihan Worker Mati (*Dead Worker Recovery*):**
   - Supervisor worker menjalankan pengecekan `XPENDING` secara berkala (interval 60 detik).
   - Jika sebuah tugas tidak diselesaikan dalam 5 menit, tugas tersebut dialihkan (*claimed*) oleh worker lain melalui `XCLAIM`.
3. **Dead-Letter Queue (DLQ):**
   - Berkas EPUB korup atau teks yang gagal diproses setelah 3 kali percobaan dipindahkan ke `stream:epub:dlq` untuk inspeksi manual kurator, tanpa menghambat antrean utama.

---

## 5. Peta Jalan Penskalaan Horisontal (Phase 2 Roadmap)

* **Autoscaling Worker Pods:** Menyesuaikan jumlah replika worker Python berdasarkan panjang antrean `XLEN stream:epub:ingestion` di Kubernetes (KEDA).
* **Multi-Node Read Replica PostgreSQL:** Memisahkan beban kueri leksikal FTS dan kueri katalog ke read replica, menjaga primary database tetap optimal untuk transaksi status baca.
* **Global Edge Caching via Cloudflare:** Menyimpan berkas bab HTML yang telah disanitasi pada cache edge Cloudflare (TTL 7 hari) untuk latensi akses pembaca <20ms di seluruh dunia.
