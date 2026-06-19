use crate::agent::{AutonomousAgent, Message};
use crate::proptech::tokenization::PropertyTokenizationEngine;
use async_trait::async_trait;

pub struct CrePortfolioAcquisitionAgent;

#[async_trait]
impl AutonomousAgent for CrePortfolioAcquisitionAgent {
    fn name(&self) -> &str { "CRE Portfolio Acquisition Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["underwrite_commercial".to_string(), "initiate_fractionalization".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let property_id = msg.payload.get("property_id").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
        let valuation = msg.payload.get("valuation").and_then(|v| v.as_u64()).unwrap_or(0);

        let engine = PropertyTokenizationEngine;
        let fractional_result = engine.fractionalize_asset(property_id, valuation).await?;

        Ok(Some(Message {
            id: format!("{}-tokenized", msg.id),
            sender: self.name().to_string(),
            recipient: "CFO".to_string(),
            payload: serde_json::json!({ "status": "FRACTIONALIZED", "log": fractional_result }),
        }))
    }
}
