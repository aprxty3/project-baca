-- Migrasi 06 Rollback: Menghapus Tabel Progres & Gamifikasi

DROP TABLE IF EXISTS saved_quotes;
DROP TABLE IF EXISTS user_badges;
DROP TABLE IF EXISTS badges;
DROP TABLE IF EXISTS reading_activity_logs;
DROP TABLE IF EXISTS user_reading_streaks;
DROP TABLE IF EXISTS user_reading_progress CASCADE;
