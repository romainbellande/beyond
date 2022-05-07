use yew::prelude::*;
use yew::Callback;
use yew_agent::Bridged;
use crate::services::event_bus::EventBus;

#[function_component(Home)]
pub fn home() -> Html {
    let state = use_state(Vec::new);

    {
        let state = state.clone();
        use_effect(move || {
            let producer = EventBus::bridge(Callback::from(move |msg| {
                let mut messages = (*state).clone();
                state.set(msg)
            }));

            || drop(producer)
        });
    }

    let output = state.iter().map(|it| html! { <p>{ if it.name.is_some() { it.clone().name.unwrap() } else { "".to_string() } }</p> });

    html! {
        <div>{ for output }</div>
    }
}
