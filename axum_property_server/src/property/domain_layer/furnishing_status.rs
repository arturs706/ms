use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum LettingClass {
    LongLet,
    ShortLet,
    Student,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FurnishedStatus {
    FullyFurnished,
    Unfurnished,
    PartlyFurnished,
    UnfurnishedFurnished,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyClass {
    pub prop_class_id: Uuid,
    pub property_id: Uuid,
    pub is_hmo: bool,
    pub letting_class: LettingClass,
    pub furnished_status: FurnishedStatus,
}
