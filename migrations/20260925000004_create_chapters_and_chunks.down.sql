-- Migration 04 Rollback: Drop book_chunks and chapters tables

DROP TABLE IF EXISTS book_chunks;
DROP TABLE IF EXISTS chapters CASCADE;
