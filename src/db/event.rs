use super::super::common_rpc_types::NodeName;
use super::types::{AgentExtra, AgentQueryResult};

pub enum Event {
    AgentUpdated {
        before: Box<Option<AgentQueryResult>>,
        after: Box<AgentQueryResult>,
    },
    ConnectionChanged {
        a