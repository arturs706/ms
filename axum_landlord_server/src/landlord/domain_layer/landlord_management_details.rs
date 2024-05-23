use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LLLetmanag {
    pub llletmanag: Uuid,
    pub llgdet: Uuid, // Assuming llgdet is the foreign key referencing llgendetails (landlord_id)
    pub payfreq: String,
    pub isexempt: bool,
    pub nrlex: String,
    pub excfrom: bool,
    pub vat: String,
    pub nin: String,
    pub utr: String,
    pub statemtemp: String,
    pub llstatemsubovr: String,
    pub llpaystate: String,
    pub accemail: String,
}

// Implement any additional functionality or methods for the LLLetmanag entity here
