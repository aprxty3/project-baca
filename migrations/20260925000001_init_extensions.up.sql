-- Migration 01: Enable PostgreSQL 17 extensions
-- Extensions: uuid-ossp (UUIDs), pg_trgm (fuzzy/trigram search), vector (semantic embeddings)

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "vector";
