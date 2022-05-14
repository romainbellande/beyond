use super::inventory::InventoryItem;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::resources::{IRON, WOOD};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct Base {
    pub id: String,
    pub inventory: Vec<InventoryItem>
}

impl Base {
    pub fn first_new() -> Self {
        Self {
            id: ObjectId::new().to_string(),
            inventory: Self::default_inventory(),
        }
    }

    pub fn default_inventory() -> Vec<InventoryItem> {
        let inventory: Vec<InventoryItem> = vec![
            InventoryItem::new(IRON.to_string(), 200),
            InventoryItem::new(WOOD.to_string(), 400),
        ];

        inventory
    }
}
