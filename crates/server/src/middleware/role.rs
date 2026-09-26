//! Role-Based Access Control (RBAC) authorization guards.

use super::auth::AuthUser;
use shared::AppError;

/// Enforces that the authenticated user possesses the 'admin' role
pub fn require_admin(user: &AuthUser) -> Result<(), AppError> {
    if user.role != "admin" {
        return Err(AppError::Forbidden(
            "Administrator privileges required to perform this action".to_string(),
        ));
    }
    Ok(())
}
