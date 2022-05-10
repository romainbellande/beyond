//! Web socket Service.
// https://blog.devgenius.io/lets-build-a-websockets-project-with-rust-and-yew-0-19-60720367399f
use beyond_core::events::{ClientEvent, ServerEvent};
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo::net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use crate::services::planets_service::{PlanetsService, Request};
use yew_agent::Dispatched;

pub struct WebsocketService {
    pub tx: Sender<ClientEvent>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:3000/ws/").unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<ClientEvent>(1000);

        let mut planets_service = PlanetsService::dispatcher();

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                log::debug!("got event from channel! {:?}", s);
                write.send(Message::Bytes(s.into_u8_array())).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log::debug!("from websocket: {}", data);
                        // event_bus.send(Request::PlanetsServiceMsg(data));
                    }
                    Ok(Message::Bytes(event_bytes)) => {
                        let event = ServerEvent::from_u8_array(&event_bytes);
                        log::debug!("from websocket: {:?}", event);
                        match event {
                            ServerEvent::GetPlanetsResponse(_) => {
                                planets_service.send(event);
                            }
                            unkown_message => {
                                info!("unkown_message: {:?}", unkown_message);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("ws: {:?}", e)
                    }
                }
            }
            log::debug!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}
