use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyBrochures {
    pub property_brochure_id: Uuid,
    pub property_id: Uuid,
    pub brochure_urls: Vec<String>,
}
