use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct ProcurementSpecialistAgent;

#[async_trait]
impl AutonomousAgent for ProcurementSpecialistAgent {
    fn name(&self) -> &str { "Procurement Specialist Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["issue_purchase_orders".to_string(), "vet_service_vendors".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let action = msg.payload.get("action").and_then(|v| v.as_str()).unwrap_or("");
        
        if action == "CREATE_VENDOR_PO" {
            let target_building = msg.payload.get("target").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
            return Ok(Some(Message {
                id: format!("{}-po-created", msg.id),
                sender: self.name().to_string(),
                recipient: "CFO".to_string(),
                payload: serde_json::json!({
                    "status": "PURCHASE_ORDER_PENDING_SIGNATURE",
                    "allocation_target": target_building,
                    "estimated_cost_usdc": 1200
                }),
            }));
        }
        Ok(None)
    }
}
