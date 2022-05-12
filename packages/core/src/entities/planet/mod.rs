use super::AppCollection;
use crate::resources::Resource;
use bson::oid::ObjectId;
use gen_planet_name::PlanetName;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct Planet {
    pub id: String,
    pub name: Option<String>,
    pub resources: Vec<Resource>,
    pub coordinates: Coordinates,
    #[serde(rename = "type")]
    pub ty: String,
}

impl Planet {
    pub fn rand_type() -> String {
        let mut rng = rand::thread_rng();
        let types: Vec<&str> = vec![
            "ocean",
            "ice",
            "exoplanet",
            "carbon",
            "desert",
            "lava",
            "iron",
            "hydrogen",
            "silicate",
            "telluric",
            "helium",
        ];
        types.choose(&mut rng).unwrap().to_string()
    }

    pub fn rand_one(filepath: String) -> Self {
        Planet {
            id: ObjectId::new().to_string(),
            coordinates: Coordinates::rand(),
            name: Some(PlanetName::new(filepath).generate()),
            resources: Resource::rand_list(),
            ty: Self::rand_type(),
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
    pub z: i16,
}

impl Coordinates {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        Coordinates {
            x: rng.gen_range(-1000..1000),
            y: rng.gen_range(-1000..1000),
            z: rng.gen_range(-10..10),
        }
    }
}
