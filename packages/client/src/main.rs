use client::start_websocket;
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    start_websocket().unwrap();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "bg-red-500", "hello, wasm!" }
    })
}
