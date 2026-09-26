//! SeaORM Entity definitions for all 13 PostgreSQL tables in Project Baca.

pub mod badges;
pub mod book_chunks;
pub mod book_tags;
pub mod books;
pub mod chapters;
pub mod reading_activity_logs;
pub mod saved_quotes;
pub mod tags;
pub mod tldr_cache;
pub mod user_badges;
pub mod user_reading_progress;
pub mod user_reading_streaks;
pub mod users;

// Convenient re-exports of models and entities
pub use badges::Entity as Badges;
pub use book_chunks::Entity as BookChunks;
pub use book_tags::Entity as BookTags;
pub use books::Entity as Books;
pub use chapters::Entity as Chapters;
pub use reading_activity_logs::Entity as ReadingActivityLogs;
pub use saved_quotes::Entity as SavedQuotes;
pub use tags::Entity as Tags;
pub use tldr_cache::Entity as TldrCache;
pub use user_badges::Entity as UserBadges;
pub use user_reading_progress::Entity as UserReadingProgress;
pub use user_reading_streaks::Entity as UserReadingStreaks;
pub use users::Entity as Users;
