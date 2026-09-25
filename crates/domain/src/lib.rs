//! Pure Domain Models, Entities, and Value Objects for Project Baca.
//! This crate does not depend on database or web framework specifics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, thiserror::Error, Clone)]
pub enum DomainError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Domain entity not found: {0}")]
    NotFound(String),

    #[error("Operation conflict: {0}")]
    Conflict(String),
}

/// User Domain Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Guest,
    Reader,
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Reader
    }
}

/// Book Domain Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub author: String,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub total_chapters: i32,
    pub word_count: i32,
    pub reading_time_minutes: i32,
    pub status: BookStatus,
    pub published_year: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookStatus {
    Draft,
    Processing,
    Published,
    Archived,
}

impl Default for BookStatus {
    fn default() -> Self {
        Self::Draft
    }
}

/// Chapter Domain Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: Uuid,
    pub book_id: Uuid,
    pub chapter_number: i32,
    pub title: String,
    pub content_html: String,
    pub word_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Reading Progress Domain Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub current_chapter_id: Uuid,
    pub cfi_position: String,
    pub percentage: f32,
    pub last_read_at: DateTime<Utc>,
}

/// Semantic Book Chunk Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookChunk {
    pub id: Uuid,
    pub book_id: Uuid,
    pub chapter_id: Uuid,
    pub chunk_index: i32,
    pub content: String,
    pub word_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Reading Streak Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingStreak {
    pub user_id: Uuid,
    pub current_streak_days: i32,
    pub longest_streak_days: i32,
    pub total_reading_minutes: i32,
    pub last_active_date: Option<chrono::NaiveDate>,
}
