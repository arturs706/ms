use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyFloorplans {
    pub property_fplans_id: Uuid,
    pub property_id: Uuid,
    pub fplans_urls: Vec<String>,
}
