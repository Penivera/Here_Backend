use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "attendees")]
pub struct Model {
    // Primary Key AND Foreign Key
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,

    #[sea_orm(default_value = 0)]
    pub rsvp_count: i32,

    // --- NEW SYNTAX: Relation ---
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    pub user: HasOne<super::user::Entity>,
}

// No `enum Relation` or `impl Related` needed here either.

impl ActiveModelBehavior for ActiveModel {}