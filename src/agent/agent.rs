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
    //            } => Some(*