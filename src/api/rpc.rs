use crate::storage::db::StateLedgerDatabase;

pub struct MatrixStateRpcServer {
    pub database_reference: StateLedgerDatabase,
}

impl MatrixStateRpcServer {
    pub fn handle_query_agent_status(&self, agent_id: &str) -> serde_json::Value {
        match self.database_reference.read_state(&format!("agent:{}:status", agent_id)) {
            Ok(Some(status)) => serde_json::json!({ "agent": agent_id, "status": status, "live": true }),
            _ => serde_json::json!({ "agent": agent_id, "status": "OFFLINE", "live": false })
        }
    }
}
