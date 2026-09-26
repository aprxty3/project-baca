//! SeaORM Entity: books

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub language: String,
    pub primary_theme: String,
    pub sub_theme: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub cover_url: String,
    #[sea_orm(column_type = "Text")]
    pub epub_storage_path: String,
    pub total_words: i32,
    pub estimated_reading_minutes: i32,
    pub source_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub source_url: Option<String>,
    pub license: String,
    pub publication_year: Option<i32>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chapters::Entity")]
    Chapters,
    #[sea_orm(has_many = "super::book_chunks::Entity")]
    BookChunks,
    #[sea_orm(has_many = "super::book_tags::Entity")]
    BookTags,
    #[sea_orm(has_many = "super::tldr_cache::Entity")]
    TldrCache,
    #[sea_orm(has_many = "super::user_reading_progress::Entity")]
    UserReadingProgress,
    #[sea_orm(has_many = "super::reading_activity_logs::Entity")]
    ReadingActivityLogs,
    #[sea_orm(has_many = "super::saved_quotes::Entity")]
    SavedQuotes,
}

impl Related<super::chapters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chapters.def()
    }
}

impl Related<super::book_chunks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookChunks.def()
    }
}

impl Related<super::book_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookTags.def()
    }
}

impl Related<super::tldr_cache::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TldrCache.def()
    }
}

impl Related<super::user_reading_progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserReadingProgress.def()
    }
}

impl Related<super::reading_activity_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReadingActivityLogs.def()
    }
}

impl Related<super::saved_quotes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SavedQuotes.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::book_tags::Relation::Tags.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::book_tags::Relation::Books.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
