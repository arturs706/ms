use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "landlord_status_type")]
pub enum LandlordStatus {
    ACTIVE,
    INACTIVE,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Landlord {
    pub landlord_id: Option<Uuid>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub company: Option<String>,
    pub phone: String,
    pub email: String,
    pub status: Option<LandlordStatus>,
    pub donotdelete: Option<NaiveDate>,
    pub createdby: Option<String>,
    pub createdat: Option<NaiveDateTime>
}
