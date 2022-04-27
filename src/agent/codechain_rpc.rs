use std::net::SocketAddr;

use jsonrpc_core::types::{Failure, Output, Success};
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;

use super::super::common_rpc_types::{BlackList, BlockId, NodeStatus, PendingParcel, StructuredLog, WhiteList};
use super::agent::{Agen