// use sea_orm::entity::prelude::*;
// use super::{attendee, event_categories, categories_join};

// pub struct AttendeeToEventCategory;

// impl Linked for AttendeeToEventCategory {
//     type FromEntity = attendee::Entity;
//     type ToEntity = event_categories::Entity;

//     fn link(&self) -> Vec<RelationDef> {
//         vec![
//             categories_join::Relation::Attendee.def(),
//             categories_join::Relation::EventCategories.def(),
//         ]
//     }
// }