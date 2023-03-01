
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use postgres;
use postgres::TlsMode;

use super::super::common_rpc_types as rpc_type;
use super::super::common_rpc_types::{NodeName, NodeStatus, StructuredLog};
use super::event::{Event, EventSubscriber};
use super::queries;
use super::types::{AgentExtra, AgentQueryResult, Connection, Connections, Error as DBError, Log, LogQueryParams};
use util;

#[derive(Debug, Clone)]
pub enum Message {
    InitializeAgent(Box<AgentQueryResult>, Sender<bool>),
    UpdateAgent(Box<AgentQueryResult>),
    GetAgent(NodeName, Sender<Option<AgentQueryResult>>),
    GetAgents(Sender<Vec<AgentQueryResult>>),
    GetConnections(Sender<Vec<rpc_type::Connection>>),
    SaveStartOption(NodeName, String, String),
    GetAgentExtra(NodeName, Sender<Option<AgentExtra>>),
    GetLogs(LogQueryParams, Sender<Vec<Log>>),
    WriteLogs(NodeName, Vec<StructuredLog>),
    GetLogTargets(Sender<Vec<String>>),
}

#[derive(Clone)]
pub struct ServiceSender {
    sender: Sender<Message>,
}

struct State {
    agent_query_result: HashMap<NodeName, AgentQueryResult>,
    connection: Connections,
}

impl State {
    pub fn new() -> Self {
        Self {
            agent_query_result: HashMap::new(),
            connection: Connections::new(),
        }
    }
}

pub struct Service {
    state: State,
    event_subscriber: Box<EventSubscriber>,
    db_conn: postgres::Connection,
}

pub struct ServiceNewArg {
    pub event_subscriber: Box<EventSubscriber>,
    pub db_user: String,
    pub db_password: String,
}

impl Service {
    fn new(
        ServiceNewArg {
            event_subscriber,
            db_user,
            db_password,
        }: ServiceNewArg,
    ) -> Self {
        let conn_uri = format!("postgres://{}:{}@localhost", db_user, db_password);

        let conn = postgres::Connection::connect(conn_uri, TlsMode::None).unwrap();
        queries::config::set_query_timeout(&conn).unwrap();
