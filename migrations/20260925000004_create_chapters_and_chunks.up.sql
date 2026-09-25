-- Migrasi 04: Klaster Struktur Bab & Vektor Semantik
-- Tabel: chapters, book_chunks

CREATE TABLE IF NOT EXISTS chapters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    chapter_number INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    word_count INTEGER NOT NULL DEFAULT 0,
    html_content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_chapters_book_number UNIQUE (book_id, chapter_number)
);

CREATE TABLE IF NOT EXISTS book_chunks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    chapter_id UUID NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    chunk_text TEXT NOT NULL,
    embedding vector(768) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indeks Klaster Bab & Vektor Semantik
CREATE INDEX IF NOT EXISTS idx_chapters_book_number ON chapters USING btree (book_id, chapter_number);
CREATE INDEX IF NOT EXISTS idx_book_chunks_lookup ON book_chunks USING btree (book_id, chapter_id, chunk_index);

-- Indeks HNSW Semantik untuk Scoped Quote Search via Jarak Kosinus
CREATE INDEX IF NOT EXISTS idx_book_chunks_hnsw_embedding ON book_chunks 
USING hnsw (embedding vector_cosine_ops) 
WITH (m = 16, ef_construction = 64);
