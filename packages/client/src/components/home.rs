use crate::services::planets_service::PlanetsService;
use beyond_core::entities::planet::Planet;
use stylist::Style;
use yew::prelude::*;
use yew::Callback;
use yew_agent::Bridged;

fn get_style_from_str(style_str: String) -> String {
    let style = Style::new(style_str).expect("Failed to create style");
    style.get_class_name().to_string()
}

fn get_planet_position_css(x: i16, y: i16) -> String {
    let style_str = format!(
        r#"
        position: absolute;
        left: {}vw;
        top: {}vh;
    "#,
        x, y
    );

    get_style_from_str(style_str)
}

#[function_component(Home)]
pub fn home() -> Html {
    let planets = use_state(Vec::<Planet>::new);
    let show_planet_details = use_state(|| None);

    {
        let planets = planets.clone();
        use_effect(move || {
            let producer = PlanetsService::bridge(Callback::from(move |msg| {
                planets.set(msg)
            }));

            || drop(producer)
        });
    }

    let output = planets.iter().map(|planet|  {
        let class = format!("h-5 w-5 rounded-full bg-emerald-600 cursor-pointer {}", get_planet_position_css(planet.coordinates.x, planet.coordinates.y));

        let on_planet_click = {
            let show_planet_details = show_planet_details.clone();
            let planet = planet.clone();
            Callback::from(move |_| {
                show_planet_details.set(Some(planet.clone().id))
            })
        };


        let show_planet_details = show_planet_details.clone();


        html! {
            <div onclick={on_planet_click} class={class} title={planet.clone().name}>
                <div class={"w-5 h-5 rounded-full"}  />
                <div class={"absolute mt-4"}>
                    {planet.clone().name}
                    if show_planet_details.is_some() && show_planet_details.as_ref().unwrap().eq(&planet.clone().id) {
                        <div>{"selected"}</div>
                    }
                </div>
            </div>
        }
    });

    html! {
        <div class={"w-screen h-screen bg-gray-700"}>
            <div>{ for output }</div>
        </div>
    }
}
