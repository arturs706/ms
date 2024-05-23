use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyLicence {
    pub propertylic_id: Uuid,
    pub property_id: Uuid,
    pub licence_expiry_date: Option<String>, // Change type to appropriate datetime type if needed
    pub licence_type: String,
    pub licence_notes: Option<String>,
}
