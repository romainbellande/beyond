use crate::entities::planet::Planet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    GetPlanets,
    GetPlanetsResponse(Vec<Planet>),
}

impl Event {
    pub fn into_u8_array(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn from_u8_array(data: &Vec<u8>) -> Self {
        bincode::deserialize(&data[..]).unwrap()
    }
}

impl Clone for Event {
    fn clone(&self) -> Event {
        match self {
            Self::GetPlanetsResponse(planets) => Event::GetPlanetsResponse(planets.to_vec()),
            Self::GetPlanets => Event::GetPlanets,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerEvent {
    GetPlanetsResponse(Vec<Planet>),
}

impl ServerEvent {
    pub fn into_u8_array(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn from_u8_array(data: &Vec<u8>) -> Self {
        bincode::deserialize(&data[..]).unwrap()
    }
}

impl Clone for ServerEvent {
    fn clone(&self) -> ServerEvent {
        match self {
            Self::GetPlanetsResponse(planets) => ServerEvent::GetPlanetsResponse(planets.to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientEvent {
    GetPlanets,
}

impl ClientEvent {
    pub fn into_u8_array(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn from_u8_array(data: &Vec<u8>) -> Self {
        bincode::deserialize(&data[..]).unwrap()
    }
}
