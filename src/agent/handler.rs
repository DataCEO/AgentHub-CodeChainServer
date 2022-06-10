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
        let jsonrpc_context = jsonrpc::Context::new(out.clone());
        agent_service
            .send(agent::Message::InitializeAgent(jsonrpc_context.clone()))
            .expect("Should success send InitializeAgent to service");
        Self {
            out,
            count,
            agent_service,
            jsonrpc_context,
        }
    }
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count.set(self.count.get() + 1);
        Ok(())
    }

    fn on_mess