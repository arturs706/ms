use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LandlordAddress {
    pub lladdr_id: Option<Uuid>,
    pub landlord_id: Option<Uuid>,
    pub addrone: String,
    pub addrtwo: Option<String>,
    pub towncity: String,
    pub county: Option<String>,
    pub postcode: String,
    pub country: String,
}