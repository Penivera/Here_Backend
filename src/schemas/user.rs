use serde::{Serialize,Deserialize};
#[allow(unused_imports)]
use validator::Validate;

#[derive(Serialize,Deserialize,Debug)]
pub enum Skill {
    EventPlanner,
    Vendor,
    Photographer,
    Dancer,
    Videographer
}

