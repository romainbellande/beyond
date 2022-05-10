use crate::resources::Resource;
use gen_planet_name::PlanetName;
use rand::Rng;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

pub trait AppCollection {
    fn get_collection_name() -> String;
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct Planet {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub resources: Vec<Resource>,
    pub coordinates: Coordinates,
}

impl Planet {
    pub fn rand_one(filepath: String) -> Self {
        Planet {
            id: ObjectId::new().to_string(),
            coordinates: Coordinates::rand(),
            name: PlanetName::new(filepath).generate(),
            resources: Resource::rand_list(),
        }
    }

    pub fn rand(filepath: String, count: u16) -> Vec<Planet> {
        let mut planets: Vec<Planet> = Vec::new();

        for _ in 0..count {
            planets.push(Planet::rand_one(filepath.clone()));
        }

        planets
    }
}

impl AppCollection for Planet {
    fn get_collection_name() -> String {
        String::from("planet")
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl Coordinates {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        Coordinates {
            x: rng.gen_range(0..100),
            y: rng.gen_range(0..100),
        }
    }
}
