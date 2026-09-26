-- Migration 03 Rollback: Drop book_tags, tags, and books tables

DROP TABLE IF EXISTS book_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS books CASCADE;
