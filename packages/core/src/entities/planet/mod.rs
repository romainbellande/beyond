use serde::{Deserialize, Serialize};
use crate::resources::Resource;
use rand::Rng;
use gen_planet_name::PlanetName;

pub trait AppCollection {
    fn get_collection_name() -> String;
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Planet {
    pub name: Option<String>,
    pub resources: Vec<Resource>,
    pub coordinates: Coordinates,
}

impl Planet {
    pub fn rand_one(filepath: String) -> Self {
        Planet {
            coordinates: Coordinates::rand(),
            name: Some(PlanetName::new(filepath).generate()),
            resources: Resource::rand_list(),
        }
    }

    pub fn rand(filepath: String, count: u16) -> Vec<Planet> {
        let mut planets: Vec<Planet> = Vec::new();

        for num in 0..count {
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
    pub z: i16
}

impl Coordinates {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        Coordinates {
            x: rng.gen_range(-100..100),
            y: rng.gen_range(-100..100),
            z: rng.gen_range(-100..100),
        }
    }
}
