use crate::entities::planet::Planet;
use crate::events::Event::{GetPlanets, GetPlanetsResponse};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    GetPlanets,
    GetPlanetsResponse(Vec<Planet>),
}

impl Event {
    pub fn into_u8_array(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().as_bytes().to_vec()
    }

    pub fn from_u8_array(data: &Vec<u8>) -> Self {
        serde_json::from_slice(data).unwrap()
    }
}

impl Clone for Event {
    fn clone(&self) -> Event {
        match self {
            Self::GetPlanetsResponse(planets) => GetPlanetsResponse(planets.to_vec()),
            Self::GetPlanets => GetPlanets,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerEvent {
    GetPlanetsResponse(Vec<Planet>),
}

impl ServerEvent {
    pub fn into_u8_array(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().as_bytes().to_vec()
    }

    pub fn from_u8_array(data: &Vec<u8>) -> Self {
        serde_json::from_slice(data).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientEvent {
    GetPlanets,
}

impl ClientEvent {
    pub fn into_u8_array(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().as_bytes().to_vec()
    }

    pub fn from_u8_array(data: &Vec<u8>) -> Self {
        serde_json::from_slice(data).unwrap()
    }
}
// use crate::entities::planet::Planet;
// use serde::{Deserialize, Serialize, de::DeserializeOwned};
// use serde_json;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WsEvent {
//     event: String,
//     data: Option<EventData>,
// }

// pub trait GetData<T> {
//     fn get_data(&self) -> T;
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct WsEventWithData {
//     event: String,
//     data: Option<EventData>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum EventData {
//     Foo,
//     CreatePlanetBase {
//         planet_id: String
//     }
// }

// impl WsEvent {
//     pub fn from_u8_array(data: &Vec<u8>) -> Option<EventData> {
//         let ws_event: WsEvent = serde_json::from_slice(&data).unwrap();

//         match ws_event.event.as_str() {
//             "CreatePlanetBase" => {
//                 let data: EventData = serde_json::from_slice(&data).unwrap();
//                 Some(data)
//             },
//             _ => None
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum ServerEvent {
//     GetPlanetsResponse {
//         event: String,
//         data: Vec<Planet>
//     },
// }

// impl ServerEvent {
//     pub fn into_u8_array(&self) -> Vec<u8> {
//         bincode::serialize(&self).unwrap()
//     }

//     pub fn from_u8_array(data: &Vec<u8>) -> Self {
//         bincode::deserialize(&data[..]).unwrap()
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum ClientEvent {
//     GetPlanets {
//         event: String,
//     },
// }

// impl ClientEvent {
//     pub fn into_u8_array(&self) -> Vec<u8> {
//         bincode::serialize(&self).unwrap()
//     }

//     pub fn from_u8_array(data: &Vec<u8>) -> Self {
//         bincode::deserialize(&data[..]).unwrap()
//     }
// }

// pub trait WithEvent {
//     fn get_event(&self) -> String;
// }

// pub struct GetPlanets;

// impl WithEvent for GetPlanets {
//     fn get_event(&self) -> String {
//         "GetPlanets".to_string()
//     }
// }

// pub struct GetPlanetsResponse {
//     pub data: Vec<Planet>,
// }

// impl WithEvent for GetPlanetsResponse {
//     fn get_event(&self) -> String {
//         "GetPlanetsResponse".to_string()
//     }
// }

// pub fn get_event_name_from_u8_array<'a, T>(data: &'a Vec<u8>) -> T
// where T: WithEvent + Deserialize<'a>
// {
//     bincode::deserialize(&data[..]).unwrap()
// }
// // #[derive(Serialize, Deserialize, Debug)]
// // pub struct WsEvent<T> {
// //     event: String,
// //     data: T
// // }

// // impl<'a, T: Serialize + Deserialize<'a>> WsEvent<T> {
// //     pub fn into_u8_array(&self) -> Vec<u8> {
// //         bincode::serialize(&self).unwrap()
// //     }

// //     pub fn from_u8_array(data: &'a Vec<u8>) -> Self {
// //         bincode::deserialize(&data[..]).unwrap()
// //     }

// //     pub fn get_client_event(&self) -> Option<ClientEvent> {
// //         match self.event.as_str() {
// //             "GetPlanets" => Some(ClientEvent::GetPlanets),
// //             unkown_event => {
// //                 println!("unkown_event {}", unkown_event);
// //                 None
// //             }
// //         }
// //     }

// //     pub fn get_server_event(&self) -> Option<ServerEvent> {
// //         match self.event.as_str() {
// //             "GetPlanets" => Some(ServerEvent::GetPlanetsResponse),
// //             unkown_event => {
// //                 println!("unkown_event {}", unkown_event);
// //                 None
// //             }
// //         }
// //     }
// // }
