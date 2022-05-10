use crate::services::planets_service::PlanetsService;
use beyond_core::entities::planet::Planet;
use stylist::Style;
use yew::prelude::*;
use yew::Callback;
use yew_agent::Bridged;

fn get_planet_position_css(x: i16, y: i16) -> String {
    let style_str = format!(
        r#"
        position: absolute;
        left: {}px;
        top: {}px;
        -webkit-mask-image: url('/public/svg/globe-alt.svg');
        mask-image: url('/public/svg/globe-alt.svg');
    "#,
        (x + 100) * 10, (y + 100) * 10
    );

    let style = Style::new(style_str).expect("Failed to create style");
    style.get_class_name().to_string()
}

#[function_component(Home)]
pub fn home() -> Html {
    let planets = use_state(Vec::<Planet>::new);
    // let show_planet_details = Option<bool>

    {
        let planets = planets.clone();
        use_effect(move || {
            let producer = PlanetsService::bridge(Callback::from(move |msg| {
                planets.set(msg)
            }));

            || drop(producer)
        });
    }

    // let on_planet_click = {
    //     Callback::from(move |_| )
    // };

    let output = planets.iter().map(|planet|  {
        let class = format!("h-5 w-5 rounded-full bg-emerald-900 cursor-pointer {}", get_planet_position_css(planet.coordinates.x, planet.coordinates.y));
        let title = if planet.name.is_some() {
            planet.clone().name.unwrap()
        } else {
            "".to_string()
        };

        html! {
            <div class={class} title={title} />
        }
    });

    html! {
        <div>{ for output }</div>
    }
}
