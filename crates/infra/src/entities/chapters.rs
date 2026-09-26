//! SeaORM Entity: chapters

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "chapters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub book_id: Uuid,
    pub chapter_number: i32,
    pub title: String,
    pub word_count: i32,
    #[sea_orm(column_type = "Text")]
    pub html_content: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Books,
    #[sea_orm(has_many = "super::book_chunks::Entity")]
    BookChunks,
    #[sea_orm(has_many = "super::tldr_cache::Entity")]
    TldrCache,
    #[sea_orm(has_many = "super::saved_quotes::Entity")]
    SavedQuotes,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl Related<super::book_chunks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookChunks.def()
    }
}

impl Related<super::tldr_cache::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TldrCache.def()
    }
}

impl Related<super::saved_quotes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SavedQuotes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
