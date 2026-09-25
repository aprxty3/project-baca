-- Migrasi 01: Mengaktifkan Ekstensi PostgreSQL 17
-- Ekstensi: uuid-ossp (identitas), pg_trgm (pencarian leksikal & toleransi saltik), vector (pgvector semantik)

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "vector";
