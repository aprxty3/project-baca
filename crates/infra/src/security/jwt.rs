//! JWT token creation, verification, and Redis refresh token rotation.

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use shared::AppError;
use uuid::Uuid;

/// JWT token payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (User ID)
    pub sub: Uuid,
    /// User email address
    pub email: String,
    /// User role (reader, admin, guest)
    pub role: String,
    /// Expiration timestamp
    pub exp: usize,
    /// Issued at timestamp
    pub iat: usize,
}

/// Generates a signed JWT access token with the specified expiration in minutes
pub fn generate_access_token(
    user_id: Uuid,
    email: &str,
    role: &str,
    secret: &str,
    expiry_minutes: u64,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp() as usize;
    let exp = now + (expiry_minutes as usize * 60);

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        role: role.to_string(),
        exp,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to sign JWT access token: {e}")))
}

/// Verifies and decodes a signed JWT access token
pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid or expired access token: {e}")))
}

/// Generates a cryptographically random refresh token
pub fn generate_refresh_token() -> String {
    format!("rt_{}_{:016x}", Uuid::new_v4(), rand::random::<u64>())
}

/// Stores a refresh token mapped to a user ID in Redis with expiration in days
pub async fn store_refresh_token(
    redis: &mut redis::aio::MultiplexedConnection,
    token: &str,
    user_id: Uuid,
    expiry_days: u64,
) -> Result<(), AppError> {
    let redis_key = format!("refresh_token:{token}");
    let ttl_seconds = expiry_days * 86400;

    let _: () = redis
        .set_ex(&redis_key, user_id.to_string(), ttl_seconds)
        .await
        .map_err(|e| {
            AppError::Internal(format!("Failed to persist refresh token in Redis: {e}"))
        })?;

    Ok(())
}

/// Validates an active refresh token, revokes it, and issues a rotated refresh token
pub async fn validate_and_rotate_refresh_token(
    redis: &mut redis::aio::MultiplexedConnection,
    old_token: &str,
    expiry_days: u64,
) -> Result<(Uuid, String), AppError> {
    let old_key = format!("refresh_token:{old_token}");
    let user_id_str: Option<String> = redis.get(&old_key).await.map_err(|e| {
        AppError::Internal(format!("Failed to retrieve refresh token from Redis: {e}"))
    })?;

    let user_id_str = match user_id_str {
        Some(s) => s,
        None => {
            return Err(AppError::Unauthorized(
                "Invalid or expired refresh token".to_string(),
            ))
        }
    };

    let user_id = Uuid::parse_str(&user_id_str).map_err(|_| {
        AppError::Unauthorized("Corrupted user ID in refresh token session".to_string())
    })?;

    // Invalidate the old token (rotation enforcement)
    let _: () = redis.del(&old_key).await.unwrap_or(());

    // Generate and persist new refresh token
    let new_token = generate_refresh_token();
    store_refresh_token(redis, &new_token, user_id, expiry_days).await?;

    Ok((user_id, new_token))
}

/// Revokes an active refresh token from Redis
pub async fn revoke_refresh_token(
    redis: &mut redis::aio::MultiplexedConnection,
    token: &str,
) -> Result<(), AppError> {
    let redis_key = format!("refresh_token:{token}");
    let _: () = redis
        .del(&redis_key)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to revoke refresh token: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        let user_id = Uuid::new_v4();
        let email = "reader@example.com";
        let role = "reader";
        let secret = "super-secret-test-jwt-key-minimum-32-chars!";

        let token = generate_access_token(user_id, email, role, secret, 15)
            .expect("Token generation should succeed");

        let claims =
            verify_access_token(&token, secret).expect("Token verification should succeed");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_jwt_invalid_secret_rejection() {
        let user_id = Uuid::new_v4();
        let secret = "secret1-for-signing-this-token-here!";
        let wrong_secret = "secret2-different-from-signing-key!";

        let token =
            generate_access_token(user_id, "user@test.local", "reader", secret, 15).unwrap();
        let result = verify_access_token(&token, wrong_secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_refresh_token_format() {
        let token = generate_refresh_token();
        assert!(token.starts_with("rt_"));
        assert!(token.len() > 30);
    }
}
