#[macro_use]
extern crate log;

mod event;
mod map;
mod websocket;

use dioxus::prelude::*;
use map::Map;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "bg-red-500", "hello, wasm!" }
        Map {}
    })
}

pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    websocket::start().unwrap();
    dioxus::web::launch(app);
}
