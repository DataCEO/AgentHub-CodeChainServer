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