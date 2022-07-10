use std::sync::mpsc::{channel, SendError, Sender};
use std::sync::Arc;
use std::thread;
use std::vec::Vec;

use parking_lot::RwLock;

use super::super::common_rpc_types::NodeName;
use super::super::db;
use super::super::jsonrpc;
use super::agent::{Agent, AgentSender};

pub struct State {
  