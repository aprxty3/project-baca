-- Migration 06: Reading progress, streaks, activity logs, badges, and saved quotes
CREATE TABLE IF NOT EXISTS user_reading_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    last_chapter_id UUID NOT NULL REFERENCES chapters(id) ON DELETE RESTRICT,
    last_anchor_cfi VARCHAR(255) NOT NULL,
    completion_percentage NUMERIC(5,2) NOT NULL DEFAULT 0.00 CHECK (completion_percentage >= 0.00 AND completion_percentage <= 100.00),
    is_finished BOOLEAN NOT NULL DEFAULT FALSE,
    last_read_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_user_book_progress UNIQUE (user_id, book_id)
);

CREATE TABLE IF NOT EXISTS user_reading_streaks (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    current_streak_days INTEGER NOT NULL DEFAULT 0,
    longest_streak_days INTEGER NOT NULL DEFAULT 0,
    total_reading_seconds BIGINT NOT NULL DEFAULT 0,
    total_xp INTEGER NOT NULL DEFAULT 0,
    last_activity_date DATE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS reading_activity_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    seconds_spent INTEGER NOT NULL,
    activity_date DATE NOT NULL DEFAULT CURRENT_DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS badges (
    id VARCHAR(50) PRIMARY KEY,
    title_id VARCHAR(100) NOT NULL,
    title_en VARCHAR(100) NOT NULL,
    description_id TEXT NOT NULL,
    description_en TEXT NOT NULL,
    icon_asset VARCHAR(100) NOT NULL,
    xp_reward INTEGER NOT NULL DEFAULT 50,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    badge_id VARCHAR(50) NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_user_badge UNIQUE (user_id, badge_id)
);

CREATE TABLE IF NOT EXISTS saved_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    chapter_id UUID NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    quote_text TEXT NOT NULL,
    image_card_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Reading progress and gamification indexes
CREATE INDEX IF NOT EXISTS idx_user_progress_active ON user_reading_progress 
USING btree (user_id, last_read_at DESC) 
WHERE is_finished = FALSE;

CREATE INDEX IF NOT EXISTS idx_reading_activity_user_date ON reading_activity_logs 
USING btree (user_id, activity_date DESC);

CREATE INDEX IF NOT EXISTS idx_user_badges_user_badge ON user_badges 
USING btree (user_id, badge_id);

CREATE INDEX IF NOT EXISTS idx_saved_quotes_user_book ON saved_quotes 
USING btree (user_id, book_id, created_at DESC);
