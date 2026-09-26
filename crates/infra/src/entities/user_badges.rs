//! SeaORM Entity: user_badges

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_badges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub badge_id: String,
    pub unlocked_at: DateTimeWithTimeZone,
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
        belongs_to = "super::badges::Entity",
        from = "Column::BadgeId",
        to = "super::badges::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Badges,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::badges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Badges.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
