use std::net::SocketAddr;
use std::ops::Drop;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use jsonrpc_core::Output;
use parking_lot::{RwLock, RwLockReadGuard};
use serde_json;
use serde_json::Value;
use ws::CloseCode as WSCloseCode;

use super::super::common_rpc_types::{
    BlockId, HardwareInfo, NodeName, NodeStatus, NodeVersion, ShellStartCodeChainRequest, ShellUpdateCodeChainRequest,
    StructuredLog,
};
use super::super::db;
use super::super::jsonrpc;
use super::super::rpc::RPCResult;
use super::codechain_rpc::CodeChainRPC;
use super::service::{Message as ServiceMessage, ServiceSender};
use super::types::{AgentGetInfoResponse, CodeChainCallRPCResponse};

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Initializing,
    Normal {
        name: NodeName,
        address: Option<SocketAddr>,
        status: NodeStatus,
    },
    Stop {
        name: NodeName,
        address: Option<SocketAddr>,
        status: NodeStatus,
        cause: StopCause,
    },
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StopCause {
    AlreadyConnected,
}

impl State {
    pub fn new() -> Self {
        State::Initializing
    }

    //    pub fn status(&self) -> Option<NodeStatus> {
    //        match self {
    //            State::Initializing => None,
    //            State::Normal {
    //                status,
    //                ..
    //            } => Some(*status),
    //            State::Stop {
    //                status,
    //                ..
    //            } => Some(*status),
    //        }
    //    }

    //    pub fn address(&self) -> Option<SocketAddr> {
    //        match self {
    //            State::Initializing => None,
    //            State::Normal {
    //                address,
    //                ..
    //            } => *address,
    //        }
    //    }
    //
    pub fn name(&self) -> Option<NodeName> {
        match self {
            State::Initializing => None,
            State::Normal {
                name,
                ..
            } => Some(name.clone()),
            State::Stop {
                name,
                ..
            } => Some(name.clone()),
        }
    }
}

#[derive(Clone)]
pub struct AgentSender {
    jsonrpc_context: jsonrpc::Context,
    state: Arc<RwLock<State>>,
}

impl AgentSender {
    pub fn new(jsonrpc_context: jsonrpc::Context, state: Arc<RwLock<State>>) -> Self {
        Self {
            jsonrpc_context,
            state,
        }
    }

    pub fn read_state(&self) -> RwLockReadGuard<State> {
        self.state.read()
    }
}

pub struct Agent {
    id: i32,
    sender: AgentSender,
    state: Arc<RwLock<State>>,
    service_sender: ServiceSender,
    closed: bool,
    db_service: db::ServiceSender,
    codechain_rpc: CodeChainRPC,
}

pub enum AgentCleanupReason {
    Error(String),
    #[allow(dead_code)]
    Success,
    AlreadyConnected,
    Unexpected,
}

impl Agent {
    fn new(
        id: i32,
        jsonrpc_context: jsonrpc::Context,
        service_sender: ServiceSender,
        db_service: db::ServiceSender,
    ) -> Self {
        let state = Arc::new(RwLock::new(State::new()));
        let sender = AgentSender::new(jsonrpc_context, Arc::clone(&state));
        Self {
            id,
            state,
            sender: sender.clone(),
            service_sender,
            closed: false,
            db_service,
            codechain_rpc: CodeChainRPC::new(sender),
        }
    }

    pub fn run_thread(
        id: i32,
        jsonrpc_context: jsonrpc::Context,
        service_sender: ServiceSender,
        db_service: db::ServiceSender,
    ) -> AgentSender {
        let mut agent = Self::new(id, jsonrpc_context, service_sender, db_service);
        let sender = agent.sender.clone();

        thread::Builder::new()
            .name(format!("agent-{}", id))
            .spawn(move || match agent.run() {
                Ok(StopCause::AlreadyConnected) => {
                    agent.clean_up(AgentCleanupReason::AlreadyConnected);
                }
                Err(err) => {
                    cerror!("Agent failed : {}", err);
                    agent.clean_up(AgentCleanupReason::Error(err));
                }
            })
            .expect("Should success running agent thread");

        sender
    }

    fn run(&mut self) -> Result<StopCause, String> {
        cinfo!("Agent-{} started", self.id);

        self.update()?;
        if let State::Stop {
            cause,
            ..
        } = *self.state.read()
        {
            return Ok(cause)
        }
        self.service_sender
            .send(