use serde::{Deserialize, Serialize};

pub const MAX_RESOURCE: u8 = 6;
pub const IRON: &str = "iron";
pub const WOOD: &str = "wood";
pub const SAND: &str = "sand";
pub const OIL: &str = "oil";

pub enum Resource {
    Iron,
    Wood,
    Sand,
    Oil,
}

impl Resource {
    pub fn to_string(&self) -> String {
        let str = match self {
            Self::Iron => IRON,
            Self::Wood => WOOD,
            Self::Sand => SAND,
            Self::Oil => OIL,
        };

        str.to_string()
    }
}
