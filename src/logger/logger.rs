use std::env;
use std::thread;

use super::atty;
use super::colored::Colorize;
use super::env_logger::filter::{Builder as FilterBuilder, Filter};
use super::time;

use log::{LevelFilter, Log, Metadata, Record};

pub struct Logger {
    fi