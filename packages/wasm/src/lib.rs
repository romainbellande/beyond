use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use beyond_core::events::ClientEvent;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Hello, {}!", name);
}

// #[wasm_bindgen(js_name = getEvent)]
// pub fn get_event() -> ClientEvent {
//     ClientEvent
// }

// #[wasm_bindgen]
// #[derive(Serialize, Deserialize, Debug)]
// pub enum ClientEvent {
//     GetPlanets,
// }

// impl ClientEvent {
//     pub fn into_u8_array(&self) -> Vec<u8> {
//         bincode::serialize(&self).unwrap()
//     }

//     pub fn from_u8_array(data: &Vec<u8>) -> Self {
//         bincode::deserialize(&data[..]).unwrap()
//     }
// }
