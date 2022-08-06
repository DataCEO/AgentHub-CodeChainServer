use std::sync::mpsc::{channel, SendError, Sender};
use std::sync::Arc;
use std::thread;
use std::vec::Vec;

use parking_lot::RwLock;

use super::super::common_rpc_types::NodeName;
use super::super::db;
use super::super::jsonrpc;
use super::agent::{Agent, AgentSender};

pub struct State {
    agents: Vec<(i32, AgentSender)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct ServiceSender {
    sender: Sender<Message>,
    state: Arc<RwLock<State>>,
}

impl ServiceSender {
    pub fn send(&self, message: Message) -> Result<(), SendError<Message>> {
        self.sender.send(message)
    }

    pub fn get_agent(&self, name: NodeName) -> Option<AgentSender> {
        let state = self.state.read();
        let find_result = state.agents.iter().find(|(_, agent)| {
            let agent_state = agent.read_state();
            match agent_state.name() {
                None => false,
                Some(agent_name) => agent_name == name,
            }
        });

        find_result.map(|(_, agent)| agent.clone())
    }
}

pub struct Service {
    state: Arc<RwLock<State>>,
    next_id: i32,
    sender: ServiceSender,
    db_service: db::ServiceSender,
}

pub enum Message {
    InitializeAgent(jsonrpc::Context),
    AddAgent(i32, AgentSender),
    RemoveAgent(i32),
}

impl Service {
    pub fn run_thread(db_service: db::ServiceSender) -> ServiceSender {
        let (tx, rx) = channel();
        let state = Arc::new(RwLock::new(State::new()));
        let service_sender = ServiceSender {
            sender: tx.clone(),
            state: state.clone(),
        };

        let mut service = Service::new(service_sender.clone(), state, db_service);

        thread::Builder::new()
            .name("agent service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::InitializeAgent(jsonrpc_context) => {
                            service.create_agent(jsonrpc_context);
                        }
                        Message::AddAgent(id, agent_sender) => {
                            service.add_agent(id, agent_sender);
                        }
                        Message::RemoveAgent(id) => {
                            service.remove_agent(id);
                        }
                    }
                }
            })
            .expect("Should success running agent service thread");

        service_sender
    }

    fn new(sender: ServiceSender, state: Arc<RwLock<State>>, db_service: db::ServiceSender) -> Self {
        Service {
            state,
            next_id: 0_i32,
            sender,
            d