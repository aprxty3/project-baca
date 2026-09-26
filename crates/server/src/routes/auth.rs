//! Authentication & User REST API Handlers (SRS 1–7)

use crate::{error::HttpError, middleware::auth::AuthUser, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use chrono::Utc;
use infra::{
    entities::users, generate_access_token, generate_and_store_otp, generate_refresh_token,
    hash_password, revoke_refresh_token, send_otp_email, store_refresh_token,
    validate_and_rotate_refresh_token, verify_and_consume_otp, verify_password,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use shared::{
    ApiResponse, AppError, ChangePasswordRequest, ErrorPayload, LoginRequest, RefreshTokenRequest,
    SignupRequest, TokenResponse, UpdateProfileRequest, UserProfileDto, VerifyOtpRequest,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

/// Converts a database user model to a public UserProfileDto
fn user_to_dto(user: &users::Model) -> UserProfileDto {
    UserProfileDto {
        id: user.id,
        display_name: user.display_name.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        avatar_url: user.avatar_url.clone(),
        is_active: user.is_active,
        created_at: user.created_at.with_timezone(&Utc),
    }
}

/// Register a new account with email OTP verification (SRS 1)
#[utoipa::path(
    post,
    path = "/api/v1/auth/signup",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "Verification OTP sent to email", body = ApiResponse<serde_json::Value>),
        (status = 409, description = "Email already registered", body = ApiResponse<ErrorPayload>)
    ),
    tag = "Authentication"
)]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignupRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let existing_user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await
        .map_err(HttpError::from)?;

    let password_hash = hash_password(&req.password).map_err(HttpError::from)?;

    if let Some(user) = existing_user {
        if user.is_active {
            return Err(HttpError(AppError::Conflict(
                "An account with this email is already registered".to_string(),
            )));
        }

        // Unverified user requesting new registration: update credentials
        let mut active: users::ActiveModel = user.into();
        active.display_name = Set(req.display_name.clone());
        active.password_hash = Set(Some(password_hash));
        active.updated_at = Set(Utc::now().into());
        active.update(&state.db).await.map_err(HttpError::from)?;
    } else {
        // Create new inactive user awaiting OTP verification
        let new_user = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(req.email.clone()),
            display_name: Set(req.display_name.clone()),
            password_hash: Set(Some(password_hash)),
            role: Set("reader".to_string()),
            avatar_url: Set(None),
            is_active: Set(false),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
        };
        new_user.insert(&state.db).await.map_err(HttpError::from)?;
    }

    let mut redis_conn = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| HttpError(AppError::Internal(format!("Redis connection failed: {e}"))))?;

    let otp = generate_and_store_otp(&mut redis_conn, &req.email)
        .await
        .map_err(HttpError::from)?;

    let _ = send_otp_email(&state.config.email, &req.email, &otp).await;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Verification OTP sent to email",
            "email": req.email
        }))),
    )
        .into_response())
}

/// Verify email registration OTP and issue initial tokens (SRS 2)
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-otp",
    request_body = VerifyOtpRequest,
    responses(
        (status = 200, description = "Verification successful, JWT tokens issued", body = ApiResponse<TokenResponse>),
        (status = 400, description = "Invalid or expired OTP", body = ApiResponse<ErrorPayload>)
    ),
    tag = "Authentication"
)]
pub async fn verify_otp(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyOtpRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let mut redis_conn = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| HttpError(AppError::Internal(format!("Redis connection failed: {e}"))))?;

    let is_valid = verify_and_consume_otp(&mut redis_conn, &req.email, &req.otp)
        .await
        .map_err(HttpError::from)?;

    if !is_valid {
        return Err(HttpError(AppError::ValidationError(
            "Invalid or expired OTP".to_string(),
        )));
    }

    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError(AppError::NotFound("User record not found".to_string())))?;

    let mut active: users::ActiveModel = user.clone().into();
    active.is_active = Set(true);
    active.updated_at = Set(Utc::now().into());
    let updated_user = active.update(&state.db).await.map_err(HttpError::from)?;

    let access_token = generate_access_token(
        updated_user.id,
        &updated_user.email,
        &updated_user.role,
        state.config.jwt_secret(),
        state.config.auth.access_expiry_minutes,
    )
    .map_err(HttpError::from)?;

    let refresh_token = generate_refresh_token();
    store_refresh_token(
        &mut redis_conn,
        &refresh_token,
        updated_user.id,
        state.config.auth.refresh_expiry_days,
    )
    .await
    .map_err(HttpError::from)?;

    let response = TokenResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: (state.config.auth.access_expiry_minutes as i64 * 60),
        user: Some(user_to_dto(&updated_user)),
    };

    Ok((StatusCode::OK, Json(ApiResponse::success(response))).into_response())
}

/// Login with email and password (SRS 3)
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authentication successful", body = ApiResponse<TokenResponse>),
        (status = 401, description = "Invalid credentials or inactive account", body = ApiResponse<ErrorPayload>)
    ),
    tag = "Authentication"
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(&state.db)
        .await
        .map_err(HttpError::from)?
        .ok_or_else(|| {
            HttpError(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ))
        })?;

    if !user.is_active {
        return Err(HttpError(AppError::Unauthorized(
            "Account not activated. Please verify OTP first.".to_string(),
        )));
    }

    let is_valid = match &user.password_hash {
        Some(hash) => verify_password(&req.password, hash).map_err(HttpError::from)?,
        None => false,
    };

    if !is_valid {
        return Err(HttpError(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        )));
    }

    let mut redis_conn = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| HttpError(AppError::Internal(format!("Redis connection failed: {e}"))))?;

    let access_token = generate_access_token(
        user.id,
        &user.email,
        &user.role,
        state.config.jwt_secret(),
        state.config.auth.access_expiry_minutes,
    )
    .map_err(HttpError::from)?;

    let refresh_token = generate_refresh_token();
    store_refresh_token(
        &mut redis_conn,
        &refresh_token,
        user.id,
        state.config.auth.refresh_expiry_days,
    )
    .await
    .map_err(HttpError::from)?;

    let response = TokenResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: (state.config.auth.access_expiry_minutes as i64 * 60),
        user: Some(user_to_dto(&user)),
    };

    Ok((StatusCode::OK, Json(ApiResponse::success(response))).into_response())
}

/// Refresh expired access token with refresh token rotation (SRS 4)
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token pair rotated successfully", body = ApiResponse<TokenResponse>),
        (status = 401, description = "Invalid or expired refresh token", body = ApiResponse<ErrorPayload>)
    ),
    tag = "Authentication"
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let mut redis_conn = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| HttpError(AppError::Internal(format!("Redis connection failed: {e}"))))?;

    let (user_id, new_refresh_token) = validate_and_rotate_refresh_token(
        &mut redis_conn,
        &req.refresh_token,
        state.config.auth.refresh_expiry_days,
    )
    .await
    .map_err(HttpError::from)?;

    let user = users::Entity::find_by_id(user_id)
        .one(&state.db)
        .await
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError(AppError::NotFound("User not found".to_string())))?;

    if !user.is_active {
        return Err(HttpError(AppError::Unauthorized(
            "Account has been deactivated".to_string(),
        )));
    }

    let access_token = generate_access_token(
        user.id,
        &user.email,
        &user.role,
        state.config.jwt_secret(),
        state.config.auth.access_expiry_minutes,
    )
    .map_err(HttpError::from)?;

    let response = TokenResponse {
        access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: (state.config.auth.access_expiry_minutes as i64 * 60),
        user: Some(user_to_dto(&user)),
    };

    Ok((StatusCode::OK, Json(ApiResponse::success(response))).into_response())
}

/// Logout and invalidate active refresh token session (SRS 5)
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Successfully logged out", body = ApiResponse<serde_json::Value>)
    ),
    tag = "Authentication"
)]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let mut redis_conn = state
        .redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| HttpError(AppError::Internal(format!("Redis connection failed: {e}"))))?;

    let _ = revoke_refresh_token(&mut redis_conn, &req.refresh_token).await;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Successfully logged out"
        }))),
    )
        .into_response())
}

/// Retrieve authenticated user profile (SRS 6)
#[utoipa::path(
    get,
    path = "/api/v1/me",
    responses(
        (status = 200, description = "User profile retrieved", body = ApiResponse<UserProfileDto>),
        (status = 401, description = "Unauthorized", body = ApiResponse<ErrorPayload>)
    ),
    security(("BearerAuth" = [])),
    tag = "User Management"
)]
pub async fn get_me(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
) -> Result<Response, HttpError> {
    let user = users::Entity::find_by_id(auth.id)
        .one(&state.db)
        .await
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError(AppError::NotFound("User not found".to_string())))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(user_to_dto(&user))),
    )
        .into_response())
}

/// Update user profile details (SRS 6)
#[utoipa::path(
    patch,
    path = "/api/v1/me",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = ApiResponse<UserProfileDto>),
        (status = 401, description = "Unauthorized", body = ApiResponse<ErrorPayload>)
    ),
    security(("BearerAuth" = [])),
    tag = "User Management"
)]
pub async fn update_me(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let user = users::Entity::find_by_id(auth.id)
        .one(&state.db)
        .await
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError(AppError::NotFound("User not found".to_string())))?;

    let mut active: users::ActiveModel = user.into();
    if let Some(display_name) = req.display_name {
        active.display_name = Set(display_name);
    }
    if let Some(avatar_url) = req.avatar_url {
        active.avatar_url = Set(Some(avatar_url));
    }
    active.updated_at = Set(Utc::now().into());

    let updated = active.update(&state.db).await.map_err(HttpError::from)?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(user_to_dto(&updated))),
    )
        .into_response())
}

/// Change password for authenticated user (SRS 7)
#[utoipa::path(
    put,
    path = "/api/v1/me/password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = ApiResponse<serde_json::Value>),
        (status = 401, description = "Invalid current password", body = ApiResponse<ErrorPayload>)
    ),
    security(("BearerAuth" = [])),
    tag = "User Management"
)]
pub async fn change_password(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Response, HttpError> {
    req.validate().map_err(HttpError::from)?;

    let user = users::Entity::find_by_id(auth.id)
        .one(&state.db)
        .await
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError(AppError::NotFound("User not found".to_string())))?;

    let is_valid = match &user.password_hash {
        Some(hash) => verify_password(&req.current_password, hash).map_err(HttpError::from)?,
        None => false,
    };

    if !is_valid {
        return Err(HttpError(AppError::Unauthorized(
            "Current password is incorrect".to_string(),
        )));
    }

    let new_hash = hash_password(&req.new_password).map_err(HttpError::from)?;
    let mut active: users::ActiveModel = user.into();
    active.password_hash = Set(Some(new_hash));
    active.updated_at = Set(Utc::now().into());
    active.update(&state.db).await.map_err(HttpError::from)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Password changed successfully"
        }))),
    )
        .into_response())
}

/// Delete account with cascading data deletion (SRS 8)
#[utoipa::path(
    delete,
    path = "/api/v1/me",
    responses(
        (status = 200, description = "Account deleted successfully", body = ApiResponse<serde_json::Value>),
        (status = 401, description = "Unauthorized", body = ApiResponse<ErrorPayload>)
    ),
    security(("BearerAuth" = [])),
    tag = "User Management"
)]
pub async fn delete_me(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
) -> Result<Response, HttpError> {
    users::Entity::delete_by_id(auth.id)
        .exec(&state.db)
        .await
        .map_err(HttpError::from)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Account successfully deleted"
        }))),
    )
        .into_response())
}

/// Assembles public authentication and user management routes
pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/signup", post(signup))
        .route("/verify-otp", post(verify_otp))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

/// Assembles user profile management routes
pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_me))
        .route("/", patch(update_me))
        .route("/", delete(delete_me))
        .route("/password", put(change_password))
        .route("/badges", get(crate::routes::list_user_badges))
}
