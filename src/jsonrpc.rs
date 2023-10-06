
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::option::Option;
use std::result::Result::{Err, Ok};
use std::sync::mpsc::{channel, RecvError, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::sync::PoisonError;
use std::time::Duration;

use jsonrpc_core::types::{
    Call, Error as JSONRPCError, ErrorCode, Failure, Id, MethodCall, Notification, Output, Params, Response, Success,
    Version,
};
use parking_lot::Mutex;
use rand;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use serde_json::{Error as SerdeError, Value};

use super::router::Error as RouterError;
use super::ws::{Error as WSError, Message, Sender as WSSender};

pub fn handle<F>(router: F, text: String) -> Option<String>