#[macro_use]
extern crate log;

mod app_state;
mod components;
mod services;
mod switch;
mod websocket;

use yew::prelude::*;
use yew_router::prelude::*;
// use beyond_core::entities::planet::Planet;
use beyond_core::events::ClientEvent;
use switch::{switch, Route};
use websocket::WebsocketService;
pub struct AppState;

#[function_component(App)]
fn app() -> Html {
    let wss = WebsocketService::new();

    let event = ClientEvent::GetPlanets;

    let result = wss.tx.clone().try_send(event).ok();

    if result.is_some() {
        log::debug!("message sent successfully");
    }

    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
