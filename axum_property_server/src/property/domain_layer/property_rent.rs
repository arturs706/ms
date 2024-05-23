use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum RentType {
    Pcm,
    Pw,
    Pd,
    Pa,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyRent {
    pub reference_id: Uuid,
    pub property_id: Uuid,
    pub rent_type: RentType,
    pub rent_cost: f64,
    pub valuation_rent: Option<f64>,
    pub min_rent: Option<f64>,
    pub deposit: Option<f64>,
    pub holding_deposit: Option<f64>,
}

// Implement any additional functionality or methods for the PropertyRent entity here
