use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
//use chrono::Utc;
use super::EventCategory;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "event_categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: EventCategory,

    pub description: Option<String>,

    #[sea_orm(default_value = "Utc::now()")]
    pub created_at: Option<DateTimeUtc>,

    #[sea_orm(default_value = "Utc::now()")]
    pub updated_at: Option<DateTimeUtc>,

    #[sea_orm(has_many, via = "categories_join")]
    pub users: HasMany<super::attendee::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}