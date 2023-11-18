
#[macro_use]
extern crate log;

extern crate chrono;
extern crate iron;
extern crate jsonrpc_core;
extern crate parking_lot;
extern crate postgres;
extern crate primitives as cprimitives;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate ws;

#[macro_use]
mod logger;
mod agent;
mod common_rpc_types;
mod db;
mod event_propagator;
mod frontend;
mod jsonrpc;
mod router;
mod rpc;
mod util;

use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use iron::prelude::*;
use iron::status;
use parking_lot::Mutex;
use ws::listen;

use self::event_propagator::EventPropagator;
use self::logger::init as logger_init;
use self::router::Router;

fn main() {
    logger_init().expect("Logger should be initialized");

    // FIXME: move to config
    let db_user = "codechain-agent-hub";
    let db_password = "preempt-entreat-bell-chanson";

    let frontend_service_sender = frontend::Service::run_thread();
    let event_propagater = Box::new(EventPropagator::new(frontend_service_sender.clone()));
    let db_service_sender = db::Service::run_thread(db::ServiceNewArg {
        event_subscriber: event_propagater,
        db_user: db_user.to_string(),
        db_password: db_password.to_string(),
    });
    let agent_service_sender = agent::Service::run_thread(db_service_sender.clone());
    let agent_service_for_frontend = agent_service_sender.clone();
    let web_handler = WebHandler::new(agent_service_sender.clone());

    let frontend_join = thread::Builder::new()
        .name("frontend listen".to_string())
        .spawn(move || {
            let count = Rc::new(Cell::new(0));
            let mut frontend_router = Arc::new(Router::new());
            frontend::add_routing(Arc::get_mut(&mut frontend_router).unwrap());
            let frontend_context = frontend::Context {
                agent_service: agent_service_for_frontend,
                db_service: db_service_sender.clone(),
                passphrase: std::env::var("PASSPHRASE").unwrap_or_else(|_| "passphrase".to_string()),
            };
            listen("0.0.0.0:3012", move |out| frontend::WebSocketHandler {
                out,
                count: count.clone(),
                context: frontend_context.clone(),
                router: frontend_router.clone(),
                frontend_service: frontend_service_sender.clone(),
            })
            .unwrap();
        })
        .expect("Should success listening frontend");

    let agent_join = thread::Builder::new()
        .name("agent listen".to_string())
        .spawn(move || {
            let count = Rc::new(Cell::new(0));