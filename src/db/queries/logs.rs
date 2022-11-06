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
                base_num + 5,
                base_num + 6
            ));
            let rfc3339with_nano_second = "%Y-%m-%dT%H:%M:%S.%f%z";
            let datetime = chrono::DateTime::parse_from_str(&log.timestamp, rfc3339with_nano_second).unwrap();
            parameters.push(Box::new(node_name));
            parameters.push(Box::new(log.level.clone()));
            parameters.push(Box::new(log.target.clone()));
            parameters.push(Box::new(log.message.clone()));
            parameters.push(Box::new(datetime));
            parameters.push(Box::new(log.thread_name.clone()));
        }

        let full_sql = format!(
     