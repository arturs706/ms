use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Headline {
    JustAdded,
    MustBeSeen,
    Refurbished,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingDetails {
    pub marketing_det_id: Uuid,
    pub property_id: Uuid,
    pub summary: Option<String>,
    pub full_description: Option<String>,
    pub headline: Headline,
    pub virtual_tour_link: Option<String>,
    pub virtual_tour_link_2: Option<String>,
}
