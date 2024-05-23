use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiarySettings {
    pub staff_id: Uuid,
    pub diary_colour: String,
    pub popup_notifi_en: bool,
    pub email_notifi_en: bool,
    pub updated_at: DateTime<Utc>,
}
