use std::borrow::Borrow;
use std::rc::Rc;

use chrono;
use postgres;
use postgres::types::ToSql;

use super::super::super::common_rpc_types::StructuredLog;
use super::super::types::OrderBy;
use super::super::types::{Log, LogQueryParams};

pub fn insert(conn: &postgres::Connection, node_name: &str, logs: Vec<StructuredLog>) -> postgres::Result<()> {
    ctrace!("Add log {} : {:?}", node_name, logs);

    if logs.is_empty() {
        return Ok(())
    }

    for log_chunk in logs.chunks(1000) {
        let mut parameters_positions: Vec<String> = Vec::new();
        let mut parameters: Vec<Box<ToSql>> = Vec::new();

        for (row_index, log) in log_chunk.iter().enumerate() {
            let base_num = row_index * 6;
            parameters_positions.push(format!(
                "(${}, ${}, ${}, ${}, ${}, ${})",
                base_num + 1,
                base_num + 2,
                base_num + 3,
                base_num + 4,
            