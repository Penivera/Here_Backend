use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "attendees")]
pub struct Model {
    // Primary Key AND Foreign Key
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,

    

    // --- NEW SYNTAX: Relation ---
    #[sea_orm(
        belongs_to,
        from = "user_id",
        to = "id",
        on_delete = "Cascade"
    )]
    pub user: HasOne<super::user::Entity>,
    /// SECTION --- NEW SYNTAX: Relation ---
    #[sea_orm(has_many)]
    pub attendance: HasMany<super::attendance::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}