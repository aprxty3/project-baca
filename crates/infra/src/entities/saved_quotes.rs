//! SeaORM Entity: saved_quotes

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "saved_quotes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub chapter_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub quote_text: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub image_card_url: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Books,
    #[sea_orm(
        belongs_to = "super::chapters::Entity",
        from = "Column::ChapterId",
        to = "super::chapters::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Chapters,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl Related<super::chapters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chapters.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
