-- Migration 07: Revert database performance optimizations and restore previous index topology

-- 1. Drop catalog ordering index
DROP INDEX IF EXISTS idx_books_published_year_id;

-- 2. Drop added foreign key indexes
DROP INDEX IF EXISTS idx_tldr_cache_chapter_id;
DROP INDEX IF EXISTS idx_user_badges_badge_id;
DROP INDEX IF EXISTS idx_saved_quotes_chapter_id;
DROP INDEX IF EXISTS idx_saved_quotes_book_id;
DROP INDEX IF EXISTS idx_reading_activity_book_id;
DROP INDEX IF EXISTS idx_user_progress_chapter_id;
DROP INDEX IF EXISTS idx_user_progress_book_id;
DROP INDEX IF EXISTS idx_book_chunks_chapter_id;

-- 3. Revert users email unique constraint and separate non-unique index
ALTER TABLE users DROP CONSTRAINT IF EXISTS idx_users_email;
ALTER TABLE users ADD CONSTRAINT users_email_key UNIQUE (email);
CREATE INDEX IF NOT EXISTS idx_users_email ON users USING btree (email);

-- 4. Recreate previously dropped redundant indexes
CREATE INDEX IF NOT EXISTS idx_user_badges_user_badge ON user_badges USING btree (user_id, badge_id);
CREATE INDEX IF NOT EXISTS idx_tldr_cache_lookup ON tldr_cache USING btree (book_id, chapter_id, recap_type);
CREATE INDEX IF NOT EXISTS idx_tags_slug ON tags USING btree (slug);
CREATE INDEX IF NOT EXISTS idx_chapters_book_number ON chapters USING btree (book_id, chapter_number);
