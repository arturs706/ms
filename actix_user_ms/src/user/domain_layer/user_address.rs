use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub address_id: Uuid,
    pub staff_id: Option<Uuid>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub town_city: String,
    pub county: Option<String>,
    pub postcode: String,
    pub country: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
