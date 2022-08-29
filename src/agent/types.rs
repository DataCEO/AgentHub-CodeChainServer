use std::net::SocketAddr;

use cprimitives::H256;
use serde_json::Value;

use super::super::common_rpc_types::{NodeName, NodeStatus};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentGetInfoResponse {
    pub status: NodeStatus,
    pub name: No