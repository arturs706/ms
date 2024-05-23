use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "property_status_type")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PropertyStatus {
    AVAILABLE,
    LETAGREED,
    LET,
    INACTIVE,
    UNDERVALUATION,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Property {
    pub property_id: Uuid,
    pub landlord_id: Uuid,
    pub secondaryland: Option<String>,
    pub branch: String,
    pub lead_staff: String,
    pub onthemarket: bool,
    pub status: Option<PropertyStatus>,
    pub dateavailable: Option<NaiveDate>,
}
