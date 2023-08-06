use std::net::SocketAddr;

use super::super::agent;
use super::super::common_rpc_types;
use super::super::common_rpc_types::{
    BlackList, BlockId, HardwareInfo, HardwareUsage, NodeName, NodeStatus, NodeVersion, PendingParcel, WhiteList,
};
use super::super::db;

#[derive(Clone)]
pub struct Context {
    pub agent_service: agent::ServiceSender,
    pub db_service: db::ServiceSender,
    pub passphrase: String,
}

pub type Event = String;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DashboardNode {
    #[serde(rename_all = "camelCase")]
    Normal {
        status: NodeStatus,
        address: Option<SocketAddr>,
        version: Option<NodeVersion>,
        best_block_id: Option<BlockId>,
        name: NodeName,
    },
    #[serde(rename_all = "camelCase")]
    #[allow(dead_code)]
    UFO {
        status: NodeStatus,
        name: NodeName,
        address: Option<SocketAddr>,
    },
}

impl DashboardNode {
    