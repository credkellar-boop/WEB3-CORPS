use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct CtoAgent;

#[async_trait]
impl AutonomousAgent for CtoAgent {
    fn name(&self) -> &str { "CTO / Technology Infrastructure Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["authorize_runtime_upgrade".to_string(), "flush_debug_buffers".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let upgrade_proposal_id = msg.payload.get("proposal_id").and_then(|v| v.as_str()).unwrap_or("");
        let verification_passed = msg.payload.get("ci_pipeline_passed").and_then(|v| v.as_bool()).unwrap_or(false);

        if verification_passed && !upgrade_proposal_id.is_empty() {
            return Ok(Some(Message {
                id: format!("{}-cto-signed", msg.id),
                sender: self.name().to_string(),
                recipient: "Appchain_Hyperchain_Engineer",
                payload: serde_json::json!({ "status": "UPGRADE_AUTHORIZED", "target_proposal": upgrade_proposal_id }),
            }));
        }
        Ok(None)
    }
}
