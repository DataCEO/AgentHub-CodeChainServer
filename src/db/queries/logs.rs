use std::borrow::Borrow;
use std::rc::Rc;

use chrono;
use postgres;
use postgres::types::ToSql;

use super::super::super::common_rpc_types::StructuredLog;
use super::super::types::OrderBy;
use super::super::types::{Log, LogQueryParams};

pub fn insert(conn: &postgres::Connection, node_name: &str, logs: Vec<StructuredLog>) -> postgres::Result<()> {
    ctrace!