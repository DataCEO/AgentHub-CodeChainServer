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

    let table_names = get_all_table_names(&conn);
    cinfo!("Table names 