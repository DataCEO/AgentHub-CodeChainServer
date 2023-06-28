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
                        Message::AddW