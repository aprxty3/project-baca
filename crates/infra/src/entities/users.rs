//! SeaORM Entity: users

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: Option<String>,
    pub display_name: String,
    pub role: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_reading_progress::Entity")]
    UserReadingProgress,
    #[sea_orm(has_one = "super::user_reading_streaks::Entity")]
    UserReadingStreak,
    #[sea_orm(has_many = "super::reading_activity_logs::Entity")]
    ReadingActivityLogs,
    #[sea_orm(has_many = "super::user_badges::Entity")]
    UserBadges,
    #[sea_orm(has_many = "super::saved_quotes::Entity")]
    SavedQuotes,
}

impl Related<super::user_reading_progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserReadingProgress.def()
    }
}

impl Related<super::user_reading_streaks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserReadingStreak.def()
    }
}

impl Related<super::reading_activity_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReadingActivityLogs.def()
    }
}

impl Related<super::user_badges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBadges.def()
    }
}

impl Related<super::saved_quotes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SavedQuotes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
