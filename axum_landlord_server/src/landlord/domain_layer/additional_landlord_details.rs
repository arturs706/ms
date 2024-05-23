use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LandlordAddDetails {
    pub llfurthdet_id: Uuid,
    pub llgdet_id: Uuid,
    pub notes: Option<String>,
    pub accholdn: Option<String>,
    pub accnr: Option<String>,
    pub sd: Option<String>,
    pub regnr: Option<String>,
}
