use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyKeysSec {
    pub property_keys_sec_id: Uuid,
    pub property_id: Uuid,
    pub key_list: Vec<i32>,
    pub code_list: Vec<String>,
}
