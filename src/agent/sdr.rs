use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct SdrAgent;

#[async_trait]
impl AutonomousAgent for SdrAgent {
    fn name(&self) -> &str { "Sales Development Rep Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["qualify_leads".to_string(), "dispatch_outreach".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let ask_price = msg.payload.get("ask_price").and_then(|v| v.as_u64()).unwrap_or(0);
        let projected_yield = msg.payload.get("projected_yield_percent").and_then(|v| v.as_f64()).unwrap_or(0.0);

        // Qualifies property opportunities based on yield performance targets
        if projected_yield >= 7.5 && ask_price <= 15_000_000 {
            return Ok(Some(Message {
                id: format!("{}-qualified", msg.id),
                sender: self.name().to_string(),
                recipient: "Account_Executive".to_string(),
                payload: serde_json::json!({ "status": "LEAD_QUALIFIED", "action": "INITIALIZE_OFFER_DRAFT" }),
            }));
        }

        Ok(None)
    }
}
