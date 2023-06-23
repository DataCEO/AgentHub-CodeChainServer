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
      