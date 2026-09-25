-- Migrasi 03: Klaster Katalog Buku & Taksonomi
-- Tabel: books, tags, book_tags

CREATE TABLE IF NOT EXISTS books (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    language VARCHAR(10) NOT NULL,
    primary_theme VARCHAR(50) NOT NULL,
    sub_theme VARCHAR(50),
    description TEXT NOT NULL,
    cover_url TEXT NOT NULL,
    epub_storage_path TEXT NOT NULL,
    total_words INTEGER NOT NULL DEFAULT 0,
    estimated_reading_minutes INTEGER NOT NULL DEFAULT 0,
    source_name VARCHAR(100) NOT NULL,
    source_url TEXT,
    license VARCHAR(50) NOT NULL DEFAULT 'Public Domain',
    publication_year INTEGER,
    status VARCHAR(20) NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL,
    slug VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS book_tags (
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, tag_id)
);

-- Indeks Klaster Katalog & Penemuan Buku (Partial Indexing WHERE status = 'published')
CREATE INDEX IF NOT EXISTS idx_books_published_fts ON books 
USING gin (to_tsvector('simple', title || ' ' || author || ' ' || description)) 
WHERE status = 'published';

CREATE INDEX IF NOT EXISTS idx_books_published_title_trgm ON books 
USING gin (title gin_trgm_ops) 
WHERE status = 'published';

CREATE INDEX IF NOT EXISTS idx_books_published_author_trgm ON books 
USING gin (author gin_trgm_ops) 
WHERE status = 'published';

CREATE INDEX IF NOT EXISTS idx_books_catalog_filter ON books 
USING btree (language, primary_theme, publication_year DESC) 
WHERE status = 'published';

-- Indeks Klaster Taksonomi & Tag
CREATE INDEX IF NOT EXISTS idx_tags_slug ON tags USING btree (slug);
CREATE INDEX IF NOT EXISTS idx_book_tags_reverse ON book_tags USING btree (tag_id, book_id);
