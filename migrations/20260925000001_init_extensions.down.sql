-- Migrasi 01 Rollback: Menghapus Ekstensi PostgreSQL 17

DROP EXTENSION IF EXISTS "vector";
DROP EXTENSION IF EXISTS "pg_trgm";
DROP EXTENSION IF EXISTS "uuid-ossp";
