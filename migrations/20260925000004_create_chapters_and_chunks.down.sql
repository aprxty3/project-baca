-- Migrasi 04 Rollback: Menghapus Tabel book_chunks, chapters

DROP TABLE IF EXISTS book_chunks;
DROP TABLE IF EXISTS chapters CASCADE;
