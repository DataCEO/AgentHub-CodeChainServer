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
    pub fn from_db_state(state: &db::AgentQueryResult) -> Self {
        DashboardNode::Normal {
            status: state.status,
            name: state.name.clone(),
            address: state.address,
            version: state.version.clone(),
            best_block_id: state.best_block_id,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnection {
    pub node_a: String,
    pub node_b: String,
}

impl NodeConnection {
    pub fn from_connection(connection: &common_rpc_types::Connection) -> Self {
        let (node_a, node_b) = connection;
        Self {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardGetNetworkResponse {
    pub nodes: Vec<DashboardNode>,
    pub connections: Vec<NodeConnection>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOption {
    pub env: String,
    pub args: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub 