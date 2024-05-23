use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyChecklists {
    pub property_clist_id: Uuid,
    pub property_id: Uuid,
    pub clist_lists: Vec<String>,
}
