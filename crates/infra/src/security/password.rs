//! Argon2id password hashing and verification.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};
use shared::AppError;

/// Hash a plaintext password using Argon2id with 64MB memory, 3 iterations, parallelism 4
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let params = Params::new(65536, 3, 4, None)
        .map_err(|e| AppError::Internal(format!("Invalid Argon2 parameters: {e}")))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let salt = SaltString::generate(&mut OsRng);

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))
}

/// Verify a plaintext password against an Argon2id hash string
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(h) => h,
        Err(_) => return Ok(false),
    };

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password_success() {
        let password = "SuperSecretPassword123!";
        let hash = hash_password(password).expect("Hashing should succeed");

        assert!(hash.starts_with("$argon2id$"));
        let valid = verify_password(password, &hash).expect("Verification should succeed");
        assert!(valid);
    }

    #[test]
    fn test_verify_password_wrong_password() {
        let password = "CorrectPassword123!";
        let hash = hash_password(password).expect("Hashing should succeed");

        let valid = verify_password("WrongPassword!", &hash).expect("Verification should succeed");
        assert!(!valid);
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let valid = verify_password("Password", "invalid_hash_string").expect("Should not panic");
        assert!(!valid);
    }
}
