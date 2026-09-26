-- Migration 01 Rollback: Drop PostgreSQL 17 extensions

DROP EXTENSION IF EXISTS "vector";
DROP EXTENSION IF EXISTS "pg_trgm";
DROP EXTENSION IF EXISTS "uuid-ossp";
