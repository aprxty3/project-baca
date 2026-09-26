//! SeaORM Entity: tags

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub slug: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::book_tags::Entity")]
    BookTags,
}

impl Related<super::book_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookTags.def()
    }
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        super::book_tags::Relation::Books.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::book_tags::Relation::Tags.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
