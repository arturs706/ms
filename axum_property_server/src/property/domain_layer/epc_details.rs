use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EpcDetails {
    pub epc_det_id: Uuid,
    pub property_id: Uuid,
    pub epcratings: String,
    pub is_exempt_epc: bool,
}
