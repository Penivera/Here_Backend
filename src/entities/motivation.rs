use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Motivation};
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "motivations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub motivation: Motivation,

   
}

impl ActiveModelBehavior for ActiveModel {}