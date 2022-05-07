use crate::entities::planet::Planet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    GetPlanets,
    GetPlanetsResponse(Vec<Planet>),
}

impl Event {
    // pub fn new(event: Event) -> Event {
    //     Event::<()> {
    //         event,
    //         data: None
    //     }
    // }

    // pub fn new_with_data(event: Event, data: T) -> Self {
    //     Event::<T> {
    //         event,
    //         data: Some(data),
    //     }
    // }

    pub fn into_u8_array(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn from_u8_array(data: Vec<u8>) -> Self {
        bincode::deserialize(&data[..]).unwrap()
    }
}
