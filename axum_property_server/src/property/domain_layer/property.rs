use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// pub enum PropertyStatus {
//     Available,
//     LetAgreed,
//     Let,
//     Inactive,
//     UnderValuation,
// }

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Property {
    pub property_id: Uuid,
    pub landlord: Uuid,
    pub secondaryland: Option<String>,
    pub branch: String,
    pub lead_staff: Option<String>,
    pub onthemarket: bool,
    pub status: String,
    pub dateavailable: Option<String>,
}
