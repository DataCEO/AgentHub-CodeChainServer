use std::net::SocketAddr;

use jsonrpc_core::types::{Failure, Output, Success};
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;

use super::super::common_rpc_types::{BlackList, BlockId, NodeStatus, PendingParcel, StructuredLog, WhiteList};
use super::agent::{AgentSender, SendAgentRPC};
use super::types::ChainGetBestBlockIdResponse;

pub struct CodeChainRPC {
    sender: AgentSender,
}

impl CodeChainRPC {
    pub fn new(sender: AgentSender) -> Self {
        Self {
            sender,
        }
    }

    pub fn get_peers(&self, status: NodeStatus) -> Result<Vec<SocketAddr>, String> {
        self.call_rpc(status, "net_getEstablishedPeers", Vec::new())
    }

    pub fn get_best_block_id(&self, status: NodeStatus) -> Result<Option<BlockId>, String> {
        le