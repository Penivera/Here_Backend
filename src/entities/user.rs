use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use super::AccountType;

#[sea_orm::model] // This macro now reads the relation attributes
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    // The "discriminator" column
    pub account_type: AccountType,

    #[sea_orm(default_value = "true")]
    pub is_active: bool,

    #[sea_orm(default_expr = "Utc::now()")]
    pub created_at: DateTimeUtc,

    #[sea_orm(default_expr = "Utc::now()")]
    pub updated_at: DateTimeUtc,
    // --- NEW SYNTAX: Relations ---
    // These fields are virtual and tell the macro to build
    // the Relation enum and Impl Related traits.

    #[sea_orm(has_one = "super::attendee::Entity")]
    pub attendee: HasOne<super::attendee::Entity>,

    #[sea_orm(has_one = "super::host::Entity")]
    pub host: HasOne<super::host::Entity>,
}

// NO MORE `enum Relation` or `impl Related` blocks.
// The `#[sea_orm::model]` macro generates them.

impl ActiveModelBehavior for ActiveModel {}