//! REST API route registration and sub-routers.

pub mod auth;
pub mod progress;

pub use auth::{auth_routes, user_routes};
pub use progress::progress_routes;
