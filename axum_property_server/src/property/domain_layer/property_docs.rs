use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDocs {
    pub property_docs_id: Uuid,
    pub property_id: Uuid,
    pub docs_urls: Vec<String>,
    pub esigned_urls: Vec<String>,
}
