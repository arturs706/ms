use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingChannels {
    pub marketing_chan_id: Uuid,
    pub property_id: Uuid,
    pub exclude_from_all_portals: bool,
    pub excluded_portals: Vec<String>,
    pub exclude_from_website: bool,
    pub feature_on_homepage: bool,
    pub exclude_from_window_rotator: Option<bool>, // It can be nullable in the database schema
}
