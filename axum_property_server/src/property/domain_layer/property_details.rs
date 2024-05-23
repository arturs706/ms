use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum PropertyType {
    Apartment,
    Bungalow,
    Cottage,
    DetachedHouse,
    DoubleRoom,
    SingleRoom,
    Penthouse,
    TerracedHouse,
    TownHouse,
    Flat,
    Maisonette,
    SemiDetachedHouse,
    Studio,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDetails {
    pub prop_det_id: Uuid,
    pub property_id: Uuid,
    pub bedrooms: String,
    pub receptions: i32,
    pub bathrooms: i32,
    pub floor_area: Option<String>,
    pub property_type: PropertyType,
}
