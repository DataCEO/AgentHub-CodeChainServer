use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::vec::Vec;

use ws;

pub struct Service {
    web_sockets: Vec<ws::Sender>,
}

pub type ServiceSender = Sender<Message>;

pub enum Message {
    AddWS(ws::Sender),
    RemoveWS(ws::Sender),
    SendEvent(String),
}

impl Service {
    pub fn run_thread() -> ServiceSender {
        let (tx, rx) = channel();
        let service_sender = tx.clone();

        let mut service = Service::new();

        thread::Builder::new()
            .name("frontend service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::SendEvent(jsonrpc_data) => {
                            service.send_event(jsonrpc_data);
                        }
                        Message::AddWS(web_socket) => {
                            service.add_ws(web_socket);
                        }
                        Message::RemoveWS(web_socket) => {
                            service.remove_ws(web_socket);
                        }
                    }
                }
            })
            .expect("Should success running agent service thread");

        service_sender
    }
}


impl Service {
    pub fn new() -> Self {
        Self {
            web_sockets: Vec::new(),
        }
    }

    pub fn send_event(&mut self, data: String) {
        for web_socket in &self.web_sockets {
            if let Err(err) = web_socket.send(data.clone()) {
                cwarn!("Error when sending event to frontend {}", err);
          