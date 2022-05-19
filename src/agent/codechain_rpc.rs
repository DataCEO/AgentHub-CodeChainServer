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
        let response: Option<ChainGetBestBlockIdResponse> =
            self.call_rpc(status, "chain_getBestBlockId", Vec::new())?;

        Ok(response.map(|response| BlockId {
            block_number: response.number,
            hash: response.hash,
        }))
    }

    pub fn version(&self, status: NodeStatus) -> Result<Option<String>, String> {
        self.call_rpc(status, "version", Vec::new())
    }

    pub fn commit_hash(&self, status: NodeStatus) -> Result<Option<String>, String> {
        self.call_rpc(status, "commitHash", Vec::new())
    }

    pub fn get_pending_parcels(&self, _status: NodeStatus) -> Result<Vec<PendingParcel>, String> {
        //        self.call_rpc(status, "chain_getPendingParcels")
        Ok(Vec::new())
    }

    pub fn get_whitelist(&self, status: NodeStatus) -> Result<Option<WhiteList>, String> {
        self.call_rpc(status, "net_getWhitelist", Vec::new())
    }

    pub fn get_blacklist(&self, status: NodeStatus) -> Result<