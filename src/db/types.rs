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
 * The Connection type shows which codechain node connects to whom.
 */
pub type Connection = (SocketAddr, SocketAddr);

/**
 * Connections type shows which codechain node connects to whom in whole netowrk's
 */
pub struct Connections {
    data: HashSet<Connection>,
}

impl Connections {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    pub fn update(
        &mut self,
        before: &AgentQueryResult,
        after: &AgentQueryResult,
    ) -> (Vec<Connection>, Vec<Connection>) {
        if before.address.is_none() || after.address.is_none() {
            return (Vec::new(), Vec::new())
        }

        let added = Self::get_added(before, after);
        let removed = Self::get_removed(before, after);

        let mut ret_added = Vec::new();
        let mut ret_removed = Vec::new();

        for added_element in added {
            if self.data.insert(added_element) {
                ret_added.push(added_element);
            }
        }

        for removed_element in removed {
            if self.data.remove(&removed_element) {
                ret_removed.push(removed_element);
            }
        }

        (ret_added, ret_removed)
    }

    fn get_added(before: &AgentQueryResult, after: &AgentQueryResult) -> Vec<Connection> {
        let before_peers: HashSet<&SocketAddr> = before.peers.iter().collect();
        after