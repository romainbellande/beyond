mod get_planets;
mod binary_message;
mod app_ws;

use actix::{StreamHandler,};
use actix_web_actors::ws;
use beyond_core::events::{ClientEvent};
use app_ws::AppWs;
use get_planets::get_planets;

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AppWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin_req)) => {
                let ws_event = ClientEvent::from_u8_array(&bin_req.to_vec());

                println!("received message: {:?}", ws_event);

                match ws_event {
                    ClientEvent::GetPlanets => {
                        get_planets(self, ctx);
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
