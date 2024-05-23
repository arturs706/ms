use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PropertyPhotos {
    pub property_photos_id: Uuid,
    pub property_id: Uuid,
    pub photo_urls: Option<Vec<String>>,
}
