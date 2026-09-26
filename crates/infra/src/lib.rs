//! Infrastructure configuration, connection pools, and database integrations for Project Baca.

pub mod config;
pub mod entities;
pub mod pool;

pub use config::{
    AiConfig, AppConfig, AuthConfig, DatabaseConfig, EmailConfig, RedisConfig, ServerConfig,
    StorageConfig,
};
pub use pool::{init_db_pool, init_redis_client};
