use std::env;
use std::thread;

use super::atty;
use super::colored::Colorize;
use super::env_logger::filter::{Builder as FilterBuilder, Filter};
use super::time;

use log::{LevelFilter, Log, Metadata, Record};

pub struct Logger {
    filter: Filter,
}

impl Logger {
    pub fn new() -> Self {
        let mut builder = FilterBuilder::new();
        builder.filter(None, LevelFilter::Info);

        if let Ok(rust_log) = env::var("RUST_LOG") {
            builder.parse(&rust_log);
        }

    