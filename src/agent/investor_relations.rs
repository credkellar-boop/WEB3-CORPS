use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct InvestorRelationsAgent;

#[async_trait]
impl AutonomousAgent for InvestorRelationsAgent {
    fn name(&self) -> &str { "Investor Relations Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["read_yield_ledgers".to_string(), "query_distribution_dates".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let inquiry = msg.payload.get("query_type").and_then(|v| v.as_str()).unwrap_or("");
        
        let response_payload = match inquiry {
            "yield_schedule" => serde_json::json!({ "frequency": "Monthly", "next_distribution": "First-Business-Day" }),
            "tax_compliance" => serde_json::json!({ "document": "Form-K1-Equivalent-On-Chain" }),
            _ => serde_json::json!({ "status": "FORWARDED_TO_HUMAN_OPERATOR" }),
        };

        Ok(Some(Message {
            id: format!("{}-ir-reply", msg.id),
            sender: self.name().to_string(),
            recipient: msg.sender,
            payload: response_payload,
        }))
    }
}
