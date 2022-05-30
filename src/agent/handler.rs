use std::cell::Cell;
use std::rc::Rc;

use ws;
use ws::{CloseCode, Error as WSError, Handler, Handshake, Result, Sender as WSSender};

use super::super::agent;
use super::super::jsonrpc;

pub struct WebSocketHandler {
    pub out: WSSender,
  