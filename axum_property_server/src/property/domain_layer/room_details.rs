use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomDetails {
    pub room_details_id: Uuid,
    pub property_id: Uuid,
    pub room_description: Option<String>,
    pub room_width: String,
    pub room_length: String,
    pub room_height: String,
}
