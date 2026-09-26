//! Authentication and security primitives (Argon2id, OTP, JWT token rotation).

pub mod jwt;
pub mod otp;
pub mod password;

pub use jwt::{
    generate_access_token, generate_refresh_token, revoke_refresh_token, store_refresh_token,
    validate_and_rotate_refresh_token, verify_access_token, Claims,
};
pub use otp::{generate_and_store_otp, hash_otp, verify_and_consume_otp};
pub use password::{hash_password, verify_password};
