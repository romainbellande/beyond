use beyond_core::{entities::planet::Planet, events::ServerEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    EventBusMsg(String),
}

pub struct PlanetsService {
    link: AgentLink<PlanetsService>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for PlanetsService {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ServerEvent;
    type Output = Vec<Planet>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            ServerEvent::GetPlanetsResponse(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.to_vec());
                }
            }
            unkown_message => {
                info!("unkown_message: {:?}", unkown_message);
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
