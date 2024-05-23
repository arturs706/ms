use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsibilityDetails {
    pub res_det_id: Uuid,
    pub property_id: Uuid,
    pub right_to_rent_res: String,
    pub maintenance_res: String,
    pub maintenance_notes: Option<String>,
}

// Implement any additional functionality or methods for the ResponsibilityDetails entity here
