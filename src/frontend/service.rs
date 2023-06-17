use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::vec::Vec;

use ws;

pub struct Service {
    web_sockets: Vec<