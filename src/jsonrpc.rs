
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
where
    F: FnOnce(String, Value) -> Result<Option<Value>, RouterError>, {
    let deserialized = serde_json::from_str(&text);
    let response: Option<Response> = match deserialized {
        Err(_) => Some(
            Failure {
                jsonrpc: None,
                id: Id::Null,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }
            .into(),
        ),
        Ok(Call::Invalid(id)) => Some(
            Failure {
                jsonrpc: None,
                id,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }
            .into(),
        ),
        Ok(Call::MethodCall(MethodCall {
            id,
            method,
            params,
            ..
        })) => {
            let value_params = serde_json::to_value(params.clone()).expect("Change to value always success");
            match router(method.clone(), value_params) {
                Ok(Some(value)) => Some(
                    Success {
                        jsonrpc: None,
                        result: value,
                        id,
                    }
                    .into(),
                ),
                Ok(None) => {
                    let mut error = JSONRPCError::new(ErrorCode::InternalError);
                    error.data = Some(serde_json::Value::String("API returns no value".to_string()));
                    Some(
                        Failure {
                            jsonrpc: None,
                            id,
                            error,
                        }
                        .into(),
                    )
                }
                Err(RouterError::MethodNotFound) => Some(
                    Failure {
                        jsonrpc: None,
                        id,
                        error: JSONRPCError::new(ErrorCode::MethodNotFound),
                    }
                    .into(),
                ),
                Err(RouterError::RPC(err)) => {
                    cwarn!("Error while handlinig {}({:#?}) : {}", method, params, err);
                    Some(
                        Failure {
                            jsonrpc: None,
                            id,
                            error: err.to_jsonrpc_error(),
                        }
                        .into(),