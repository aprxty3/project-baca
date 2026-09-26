//! Infrastructure configuration, connection pools, and database integrations for Project Baca.

pub mod config;
pub mod email;
pub mod entities;
pub mod pool;
pub mod security;

pub use config::{
    AiConfig, AppConfig, AuthConfig, DatabaseConfig, EmailConfig, RedisConfig, ServerConfig,
    StorageConfig,
};
pub use email::send_otp_email;
pub use pool::{init_db_pool, init_redis_client};
pub use security::{
    generate_access_token, generate_and_store_otp, generate_refresh_token, hash_otp, hash_password,
    revoke_refresh_token, store_refresh_token, validate_and_rotate_refresh_token,
    verify_access_token, verify_and_consume_otp, verify_password, Claims,
};
