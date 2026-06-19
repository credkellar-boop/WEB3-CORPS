use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct OperationsManagerAgent;

#[async_trait]
impl AutonomousAgent for OperationsManagerAgent {
    fn name(&self) -> &str { "Operations Manager" }
    fn role_permissions(&self) -> Vec<String> { vec!["allocate_tickets".to_string(), "approve_vendor_payout".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let action = msg.payload.get("action").and_then(|v| v.as_str()).unwrap_or("");
        
        if action == "DISPATCH_MAINTENANCE" {
            // Evaluates and targets closest field automation nodes or third-party engineering providers
            return Ok(Some(Message {
                id: format!("{}-dispatched", msg.id),
                sender: self.name().to_string(),
                recipient: "Procurement_Specialist".to_string(),
                payload: serde_json::json!({ "action": "CREATE_VENDOR_PO", "target": msg.payload.get("target_building") }),
            }));
        }
        Ok(None)
    }
}
