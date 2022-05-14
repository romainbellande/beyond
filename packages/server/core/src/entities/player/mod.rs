use super::AppCollection;
use crate::utils::hash::hash_password;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

mod inventory;
mod base;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct PlayerDto {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct PlayerWithoutPassword {
    pub id: String,
    pub username: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

impl AppCollection for Player {
    fn get_collection_name() -> String {
        String::from("user")
    }
}

impl Player {
    pub fn new(dto: PlayerDto) -> Self {
        let password_hash = hash_password(dto.password).unwrap();
        Self {
            id: ObjectId::new().to_string(),
            username: dto.username,
            password_hash,
        }
    }

    pub fn get(&self) -> PlayerWithoutPassword {
        PlayerWithoutPassword {
            id: self.id.clone(),
            username: self.username.clone(),
        }
    }
}
