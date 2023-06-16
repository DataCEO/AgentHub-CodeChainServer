
use std::cell::Cell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

use ws;
use ws::{CloseCode, Error as WSError, ErrorKind, Handler, Handshake, Result, Sender};

use super::super::jsonrpc;
use super::super::router::Router;
use super::types::Context;

#[derive(Debug)]
struct CustomError {}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error")
    }