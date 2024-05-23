use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyFeatures {
    pub property_features_id: Uuid,
    pub property_id: Uuid,
    pub features_list: Vec<String>,
}
