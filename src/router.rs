
use std::collections::HashMap;

use serde::de::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;

use super::rpc::{RPCError, RPCResponse};

pub trait Route {
    type Context;
    fn run(&self, context: Self::Context, value: Value) -> RPCResponse<Value>;
}

pub struct Router<C> {
    table: HashMap<&'static str, Box<Route<Context = C>>>,
}

impl<Arg, Result, C> Route for fn(context: C, Arg) -> RPCResponse<Result>
where
    Result: Serialize,
    for<'de> Arg: Deserialize<'de>,
{
    type Context = C;
    fn run(&self, context: Self::Context, value: Value) -> RPCResponse<Value> {
        let arg = serde_json::from_value(value)?;
        let result = self(context, arg)?;
        if let Some(result) = result {
            Ok(Some(serde_json::to_value(result)?))
        } else {
            Ok(None)
        }
    }
}

impl<Result, C> Route for fn(context: C) -> RPCResponse<Result>
where
    Result: Serialize,
{
    type Context = C;
    fn run(&self, context: Self::Context, _value: Value) -> RPCResponse<Value> {
        let result = self(context)?;
        if let Some(result) = result {
            let value_result = serde_json::to_value(result)?;
            Ok(Some(value_result))