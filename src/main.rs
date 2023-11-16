
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