//! HTTP Middleware modules for authentication, authorization, and rate limiting.

pub mod auth;
pub mod rate_limit;
pub mod role;

pub use auth::AuthUser;
pub use rate_limit::rate_limit_middleware;
pub use role::require_admin;
