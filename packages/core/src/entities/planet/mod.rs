use crate::resources::Resource;
use gen_planet_name::PlanetName;
use rand::Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys;

pub trait AppCollection {
    fn get_collection_name() -> String;
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct Planet {
    pub coordinates: Coordinates,
    name: Option<String>,
    resources: Vec<Resource>,
}

#[wasm_bindgen]
impl Planet {
    #[wasm_bindgen(getter)]
    pub fn resources(&self) -> js_sys::Array {
        js_sys::Array::from(self.resources[..])
    }

    pub fn rand_one(filepath: String) -> Self {
        Planet {
            coordinates: Coordinates::rand(),
            name: Some(PlanetName::new(filepath).generate()),
            resources: Resource::rand_list(),
        }
    }

    pub fn rand(filepath: String, count: u16) -> js_sys::Array {
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

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq, Copy)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl Coordinates {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        Coordinates {
            x: rng.gen_range(-100..100),
            y: rng.gen_range(-100..100),
        }
    }
}
