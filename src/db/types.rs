use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;

use super::super::common_rpc_types::{
    BlackList, BlockId, HardwareInfo, NodeName, NodeStatus, NodeVersion, PendingParcel, WhiteList,
};


#[derive(PartialEq, Clone, Debug, Default)]
pub struct AgentQueryResult {
    pub name: NodeName,
    pub status: NodeStatus,
    pub address: Option<SocketAddr>,
    pub peers: Vec<SocketAddr>,
    pub best_block_id: Option<BlockId>,
    pub version: Option<NodeVersion>,
    pub pending_parcels: Vec<PendingParcel>,
    pub whitelist: Option<WhiteList>,
    pub blacklist: Option<BlackList>,
    pub hardware: Option<HardwareInfo>,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct AgentExtra {
    pub prev_env: String,
    pub prev_args: String,
}

/**
 * T