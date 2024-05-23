use chrono;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]

pub struct StaffUser {
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub username: String,
    pub mob_phone: Option<String>,
    pub passwd: String,
    pub acc_level: Option<String>,
    pub status: Option<String>,
    pub a_created: Option<NaiveDateTime>,
}
