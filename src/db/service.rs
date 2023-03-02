
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

        Self {
            state: State::new(),
            event_subscriber,
            db_conn: conn,
        }
    }

    pub fn run_thread(arg: ServiceNewArg) -> ServiceSender {
        let (tx, rx) = channel();
        let service_sender = ServiceSender::new(tx.clone());

        let mut service = Service::new(arg);

        thread::Builder::new()
            .name("db service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::InitializeAgent(agent_query_result, callback) => {
                            service.initialize_agent(&agent_query_result, callback);
                        }
                        Message::UpdateAgent(agent_query_result) => {
                            service.update_agent(*agent_query_result);
                        }
                        Message::GetAgent(node_name, callback) => {
                            service.get_agent(&node_name, callback);
                        }
                        Message::GetAgents(callback) => {
                            service.get_agents(callback);
                        }
                        Message::GetConnections(callback) => {
                            service.get_connections(callback);
                        }
                        Message::SaveStartOption(node_name, env, args) => {
                            util::log_error(&node_name, service.save_start_option(node_name.clone(), &env, &args));
                        }
                        Message::GetAgentExtra(node_name, callback) => {
                            util::log_error(&node_name, service.get_agent_extra(&node_name, callback));
                        }
                        Message::GetLogs(params, callback) => {
                            let result = service.get_logs(params, callback);
                            if let Err(err) = result {
                                cerror!("Error at {}", err);
                            }
                        }
                        Message::WriteLogs(node_name, logs) => {
                            let result = service.write_logs(&node_name, logs);
                            if let Err(err) = result {
                                cerror!("Error at {}", err);
                            }
                        }
                        Message::GetLogTargets(callback) => {
                            let result = service.get_log_targets(callback);
                            if let Err(err) = result {
                                cerror!("Error at {}", err);
                            }
                        }
                    }
                }
            })
            .expect("Should success running db service thread");

        service_sender
    }

    fn initialize_agent(&mut self, state: &AgentQueryResult, callback: Sender<bool>) {
        let name = state.name.clone();
        let before = match self.state.agent_query_result.entry(name) {
            Entry::Occupied(mut before) => before.into_mut(),
            Entry::Vacant(e) => {
                self.event_subscriber.on_event(Event::AgentUpdated {
                    before: None.into(),
                    after: state.clone().into(),
                });
                e.insert(state.clone());
                if let Err(err) = callback.send(true) {
                    cerror!("Cannot send callback : {}", err);