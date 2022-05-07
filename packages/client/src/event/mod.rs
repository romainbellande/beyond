use beyond_core::events::Event;

pub fn event_matcher(event: Event) {
    match event {
        Event::GetPlanetsResponse(planets) => {
            println!("planets: {:?}", planets);
        }
        _ => {
            println!("no data");
        }
    }
}
