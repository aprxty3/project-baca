-- Migrasi 05: Klaster Wawasan & Ringkasan Atomik AI
-- Tabel: tldr_cache

CREATE TABLE IF NOT EXISTS tldr_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    chapter_id UUID REFERENCES chapters(id) ON DELETE CASCADE,
    recap_type VARCHAR(30) NOT NULL,
    content_json JSONB NOT NULL,
    model_version VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_tldr_cache UNIQUE NULLS NOT DISTINCT (book_id, chapter_id, recap_type)
);

-- Indeks Pengambilan Cepat Cache Wawasan Atomik
CREATE INDEX IF NOT EXISTS idx_tldr_cache_lookup ON tldr_cache USING btree (book_id, chapter_id, recap_type);
