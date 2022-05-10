use std::ops::Deref;

use crate::planets::PlanetRepository;
use actix::prelude::*;
use actix::{Actor, AsyncContext, StreamHandler, WrapFuture};
use actix_web::web::Bytes;
use actix_web_actors::ws;
use beyond_core::events::{ClientEvent, ServerEvent};
use log::info;
use mongodb::Database;

pub struct AppWs {
    pub db: Database,
}

impl Actor for AppWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AppWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin_req)) => {
                let ws_event = ClientEvent::from_u8_array(&bin_req.to_vec());

                match ws_event {
                    ClientEvent::GetPlanets => {
                        let db = self.db.clone();
                        let planet_repository = PlanetRepository::new(db);

                        let recipient = ctx.address().recipient();

                        let future = async move {
                            println!("request: get planet");
                            let planets = planet_repository.find().await;

                            if let Some(planets) = planets {
                                info!("planets: {:?}", planets);
                                let bin_res = ServerEvent::GetPlanetsResponse(planets).into_u8_array();
                                recipient.do_send(BinaryMessage(bin_res));
                            }
                        };

                        future.into_actor(self).spawn(ctx);
                    }
                    unknown_request => {
                        print!("unknown request: {:?}", unknown_request);
                    }
                };
            }
            _ => (),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct BinaryMessage(Vec<u8>);

impl Deref for BinaryMessage {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Bytes> for BinaryMessage {
    fn into(self) -> Bytes {
        self.0.into()
    }
}

impl StreamHandler<Result<BinaryMessage, ws::ProtocolError>> for AppWs {
    fn handle(&mut self, msg: Result<BinaryMessage, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(bin) => ctx.binary(bin),
            _ => (), //Handle errors
        }
    }
}

impl Handler<BinaryMessage> for AppWs {
    type Result = ();

    fn handle(&mut self, msg: BinaryMessage, ctx: &mut Self::Context) {
        ctx.binary(msg);
    }
}
