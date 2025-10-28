use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Motivation,EventCategory,EventType};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "User Preferences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    #[sea_orm(column_type = "Json")]
    pub preferred_event_types: Vec<EventType>,
    #[sea_orm(column_type = "Json")]
    pub preferred_event_categories: Vec<EventCategory>,
    pub attendance_aim: Motivation,
    
}

impl ActiveModelBehavior for ActiveModel {}