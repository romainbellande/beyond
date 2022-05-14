use std::ops::Deref;
use actix_web::web::Bytes;
use actix::prelude::*;
use super::app_ws::AppWs;
use actix_web_actors::ws;

#[derive(Message)]
#[rtype(result = "()")]
pub struct BinaryMessage(pub Vec<u8>);

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
