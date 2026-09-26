//! REST API route registration and sub-routers.

pub mod auth;
pub mod books;
pub mod gamification;
pub mod progress;

pub use auth::{auth_routes, user_routes};
pub use books::{
    books_routes, get_book, get_chapter, get_offline_bundle, list_books, search_books,
};
pub use gamification::{gamification_routes, list_badges, list_user_badges, record_heartbeat};
pub use progress::{get_active_progress, merge_guest_progress, progress_routes, update_progress};
