use crate::agent::{AutonomousAgent, Message};
use crate::security::guardrails::SecurityShield;
use async_trait::async_trait;

pub struct CfoAgent {
    pub multi_sig_threshold: u32,
}

#[async_trait]
impl AutonomousAgent for CfoAgent {
    fn name(&self) -> &str { "CFO Treasury Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["sign_treasury_tx".to_string(), "approve_dividend".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let action = msg.payload.get("action").and_then(|v| v.as_str()).unwrap_or("");
        
        // Enforce RBAC validation on high-privilege incoming financial messages
        if !SecurityShield::verify_rbac(&msg.sender, action) {
            return Ok(Some(Message {
                id: format!("{}-reply", msg.id),
                sender: self.name().to_string(),
                recipient: msg.sender,
                payload: serde_json::json!({"status": "REJECTED", "reason": "Unauthorized RBAC escalation attempt"}),
            }));
        }

        Ok(Some(Message {
            id: format!("{}-reply", msg.id),
            sender: self.name().to_string(),
            recipient: msg.sender,
            payload: serde_json::json!({"status": "APPROVED", "action": action}),
        }))
    }
}
