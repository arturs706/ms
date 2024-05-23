use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyPhotos {
    pub property_photos_id: Uuid,
    pub property_id: Uuid,
    pub photo_urls: Vec<String>,
}
