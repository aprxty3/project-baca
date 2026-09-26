-- Migration 07: Database performance optimization, redundant index cleanup, and foreign key indexing
-- Addresses:
-- 1. Redundant B-tree index cleanup on tables with existing UNIQUE constraints.
-- 2. Elimination of duplicate non-unique email index in favor of a single unified unique index.
-- 3. Missing foreign key indexes to accelerate cascading deletes and table joins.
-- 4. In-memory sort elimination on default catalog pagination (publication_year DESC, id ASC).

-- 1. Remove redundant duplicate indexes
DROP INDEX IF EXISTS idx_chapters_book_number;
DROP INDEX IF EXISTS idx_tags_slug;
DROP INDEX IF EXISTS idx_tldr_cache_lookup;
DROP INDEX IF EXISTS idx_user_badges_user_badge;

-- Consolidate duplicate users email indexes into a single named unique index
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_email_key;
DROP INDEX IF EXISTS idx_users_email;
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email ON users USING btree (email);
ALTER TABLE users ADD CONSTRAINT idx_users_email UNIQUE USING INDEX idx_users_email;

-- 2. Add missing foreign key indexes on referencing columns
CREATE INDEX IF NOT EXISTS idx_book_chunks_chapter_id ON book_chunks USING btree (chapter_id);
CREATE INDEX IF NOT EXISTS idx_user_progress_book_id ON user_reading_progress USING btree (book_id);
CREATE INDEX IF NOT EXISTS idx_user_progress_chapter_id ON user_reading_progress USING btree (last_chapter_id);
CREATE INDEX IF NOT EXISTS idx_reading_activity_book_id ON reading_activity_logs USING btree (book_id);
CREATE INDEX IF NOT EXISTS idx_saved_quotes_book_id ON saved_quotes USING btree (book_id);
CREATE INDEX IF NOT EXISTS idx_saved_quotes_chapter_id ON saved_quotes USING btree (chapter_id);
CREATE INDEX IF NOT EXISTS idx_user_badges_badge_id ON user_badges USING btree (badge_id);
CREATE INDEX IF NOT EXISTS idx_tldr_cache_chapter_id ON tldr_cache USING btree (chapter_id) WHERE chapter_id IS NOT NULL;

-- 3. Add default catalog pagination index (zero-sort query optimization)
CREATE INDEX IF NOT EXISTS idx_books_published_year_id ON books USING btree (publication_year DESC NULLS LAST, id ASC) WHERE status = 'published';
