use std::cell::Cell;
use std::rc::Rc;

use ws;
use ws::{CloseCode, Error as WSError, Handler, Handshake, Result, Sender as WSSender};

use super::super::agent;
use super::super::jsonrpc;

pub struct WebSocketHandler {
    pub out: WSSender,
    pub count: Rc<Cell<u32>>,
    pub agent_service: agent::ServiceSender,
    pub jsonrpc_context: jsonrpc::Context,
}

impl WebSocketHandler {
    pub fn new(out: WSSender, count: Rc<Cell<u32>>, agent_service: agent::ServiceSender) -> Self {
        let jsonrpc_c