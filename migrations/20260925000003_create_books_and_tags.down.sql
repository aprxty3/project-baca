-- Migrasi 03 Rollback: Menghapus Tabel book_tags, tags, books

DROP TABLE IF EXISTS book_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS books CASCADE;
