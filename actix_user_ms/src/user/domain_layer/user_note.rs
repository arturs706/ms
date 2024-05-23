use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub note_id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub note_text: String,
}
