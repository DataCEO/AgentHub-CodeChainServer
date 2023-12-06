use std::error::Error;
use std::fmt;
use std::result::Result;

use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError, Value};

use super::db::Error as DBError;
use super::jsonrpc;

pub type RPCResponse<T> = Result<Option<T>, RPCError>;

pub type RPCResult<T> = Result<T, RPCError>;

pub enum RPCError {
    Internal(String),
    FromAgent(JSONRPCError),
    FromDB(DBError),

    AgentNotFound,
}

impl fmt::Display for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RPCError::Internal(err) => write!(f, "RPCError {}", err),
            RPCError::FromAgent(err) => write!(f, "JSONRPCError from Agent {:?}", err),
            RPCError::FromDB(err) => write!(f, "JSONRPCError from DB {:?}", err),
            RPCError::AgentNotFound => write!(f, "Agent not found"),
        }
    }
}

impl fmt::Debug for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for RPCError {}

pub fn response<T>(value: T) -> RPCResponse<T> {
    Ok(Some(value))
}

const ERR_AGENT_NOT_FOUND: i64 = -1;

impl RPCError {
    pub fn to_jsonrpc_error(&self) -> JSONRPCError {
        match self {
           