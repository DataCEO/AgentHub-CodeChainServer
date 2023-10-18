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

        Self {
            filter: builder.build(),
        }
    }

    pub fn filter(&self) -> LevelFilter {
        self.filter.filter()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.filter.matches(record) {
            let thread_name = thread::current().name().unwrap_or_default().to_string();
            let timestamp = time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap();

            let stderr_