use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateExp {
    pub certificate_exp_id: Uuid,
    pub property_id: Uuid,
    pub gas_exp_date: Option<String>, // Change type to appropriate datetime type if needed
    pub pat_exp_date: Option<String>, // Change type to appropriate datetime type if needed
    pub electricity_exp_date: Option<String>, // Change type to appropriate datetime type if needed
    pub epc_exp_date: Option<String>, // Change type to appropriate datetime type if needed
    pub fire_alarm_exp: Option<String>, // Change type to appropriate datetime type if needed
    pub emergency_light_exp: Option<String>, // Change type to appropriate datetime type if needed
    pub smoke_alarm_exp: Option<String>, // Change type to appropriate datetime type if needed
}
