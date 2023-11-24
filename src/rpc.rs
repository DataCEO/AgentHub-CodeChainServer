use std::error::Error;
use std::fmt;
use std::result::Result;

use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError,