use actix::AsyncContext;
use beyond_core::events::ServerEvent;
use actix_web_actors::ws;
use actix::prelude::*;
use crate::planets::PlanetRepository;
use super::AppWs;
use super::binary_message::BinaryMessage;

pub fn get_planets(app_ws: &mut AppWs, ctx: &mut ws::WebsocketContext<AppWs>) {
    let db = app_ws.db.clone();
    let planet_repository = PlanetRepository::new(db);

    let recipient = ctx.address().recipient();

    let future = async move {
        println!("request: get planet");
        let planets = planet_repository.find().await;

        if let Some(planets) = planets {
            let bin_res =
                ServerEvent::GetPlanetsResponse(planets).into_u8_array();
            recipient.do_send(BinaryMessage(bin_res));
        }
    };

    future.into_actor(app_ws).spawn(ctx);
}