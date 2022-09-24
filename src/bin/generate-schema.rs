
#[macro_use]
extern crate codechain_agent_hub as chub;
extern crate postgres;
#[macro_use]
extern crate log;

use chub::logger_init;
use postgres::{Connection, TlsMode};

fn main() {
    logger_init().expect("Logger should be initialized");

    // FIXME: move to configuration file
    let user = "codechain-agent-hub";
    let password = "preempt-entreat-bell-chanson";
    let conn_uri = format!("postgres://{}:{}@localhost", user, password);
    let conn = Connection::connect(conn_uri, TlsMode::None).unwrap();

    create_agent_extra_schema(&conn);
    create_logs_schema(&conn);
}

fn create_agent_extra_schema(conn: &Connection) {
    cinfo!("Create agent_extra table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_extra (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL UNIQUE,
        prev_env VARCHAR NOT NULL,
        prev_args VARCHAR NOT NULL
    )",
        &[],
    )
    .unwrap();
}

fn create_logs_schema(conn: &Connection) {
    cinfo!("Create logs table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        level VARCHAR NOT NULL,
        target VARCHAR NOT NULL,