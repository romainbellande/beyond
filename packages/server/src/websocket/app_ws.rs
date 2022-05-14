use mongodb::Database;
use actix::Actor;
use actix_web_actors::ws;
use actix::prelude::*;
use super::binary_message::BinaryMessage;

pub struct AppWs {
    pub db: Database,
}

impl Actor for AppWs {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<BinaryMessage> for AppWs {
    type Result = ();

    fn handle(&mut self, msg: BinaryMessage, ctx: &mut Self::Context) {
        ctx.binary(msg);
    }
}
