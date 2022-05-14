use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct InventoryItem {
    pub reference: String,
    pub quantity: u32
}

impl InventoryItem {
    pub fn new(reference: String, quantity: u32) -> Self  {
        Self {
            reference,
            quantity,
        }
    }
}
