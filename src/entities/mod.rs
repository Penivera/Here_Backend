pub mod user;
pub mod event;
pub mod host;
pub mod attendee;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// --- 1. Define the shared AccountType enum ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(10))")]
pub enum AccountType {
    #[sea_orm(string_value = "Attendee")]
    Attendee,
    #[sea_orm(string_value = "Host")]
    Host,
}

