use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyAddress {
    pub address_id: Uuid,
    pub property_id: Uuid,
    pub display_add: String,
    pub add_line1: String,
    pub add_line2: Option<String>,
    pub towncity: String,
    pub county: Option<String>,
    pub postcode: String,
    pub country: String,
    pub searchable_areas: Option<String>,
}
