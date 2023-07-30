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
    pub passphrase: St