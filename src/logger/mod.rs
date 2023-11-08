extern crate atty;
extern crate colored;
extern crate env_logger;
extern crate time;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::module_inception))]
mod logger;
#[macro_use]
pub mod m