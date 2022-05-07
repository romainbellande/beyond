use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct MapProps {}

pub fn Map(cx: Scope<MapProps>) -> Element {
    cx.render(rsx! {
        div {
            "My Map"
        }
    })
}
