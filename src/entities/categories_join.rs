use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(DeriveEntityModel, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "categories_join")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i32,

    #[sea_orm(belongs_to, from = "user_id", to = "user_id")]
    pub user: Option<super::attendee::Entity>,

    #[sea_orm(belongs_to, from = "category_id", to = "id")]
    pub category: Option<super::event_categories::Entity>,
}



impl ActiveModelBehavior for ActiveModel {}