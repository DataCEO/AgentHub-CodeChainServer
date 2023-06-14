
use serde_json;
use serde_json::Value;

use super::db;
use super::frontend;
use super::jsonrpc;

pub struct EventPropagator {
    frontend_service: frontend::ServiceSender,
}

impl EventPropagator {
    pub fn new(frontend_service: frontend::ServiceSender) -> Self {
        EventPropagator {
            frontend_service,
        }
    }
}

impl db::EventSubscriber for EventPropagator {
    fn on_event(&self, event: db::Event) {
        match event {
            db::Event::AgentUpdated {
                before,
                after,
            } => {
                let mut diff = json!({
                    "name": after.name,
                });

                let after = *after;
                if before.is_none() {
                    diff["address"] = serde_json::to_value(after.address).unwrap();
                    diff["status"] = serde_json::to_value(after.status).unwrap();
                    diff["peers"] = serde_json::to_value(after.peers).unwrap();
                    diff["bestBlockId"] = serde_json::to_value(after.best_block_id).unwrap();
                    diff["version"] = serde_json::to_value(after.version).unwrap();
                    diff["pendingParcels"] = serde_json::to_value(after.pending_parcels).unwrap();
                    diff["whitelist"] = serde_json::to_value(after.whitelist).unwrap();
                    diff["blacklist"] = serde_json::to_value(after.blacklist).unwrap();
                    diff["hardware"] = serde_json::to_value(after.hardware).unwrap();
                } else {
                    let before = before.unwrap();
                    if before == after {
                        return
                    }

                    if before.address != after.address {
                        diff["address"] = serde_json::to_value(after.address).unwrap();
                    }
                    if before.status != after.status {
                        diff["status"] = serde_json::to_value(after.status).unwrap();
                    }