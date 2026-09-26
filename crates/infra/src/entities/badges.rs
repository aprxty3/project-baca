//! SeaORM Entity: badges

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "badges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title_id: String,
    pub title_en: String,
    #[sea_orm(column_type = "Text")]
    pub description_id: String,
    #[sea_orm(column_type = "Text")]
    pub description_en: String,
    pub icon_asset: String,
    pub xp_reward: i32,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_badges::Entity")]
    UserBadges,
}

impl Related<super::user_badges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBadges.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
