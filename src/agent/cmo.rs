use crate::agent::{AutonomousAgent, Message};
use crate::marketing::sentiment::SocialSentimentEngine;
use async_trait::async_trait;

pub struct CmoAgent;

#[async_trait]
impl AutonomousAgent for CmoAgent {
    fn name(&self) -> &str { "CMO / Marketing Growth Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["allocate_marketing_budget".to_string(), "deploy_campaign".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let channel_text = msg.payload.get("social_feed_snippet").and_then(|v| v.as_str()).unwrap_or("");
        
        let engine = SocialSentimentEngine;
        let score = engine.score_content_stream(channel_text);

        // Dynamically scales active marketing spends if community momentum dips
        let budget_adjustment = if score < 0.4 { 2500u64 } else { 0u64 };

        Ok(Some(Message {
            id: format!("{}-cmo-action", msg.id),
            sender: self.name().to_string(),
            recipient: "CFO".to_string(),
            payload: serde_json::json!({
                "action": "REQUEST_BUDGET_RELEASE",
                "department": "Growth_Marketing",
                "amount_usdc": budget_adjustment,
                "reason": "Social sentiment drop mitigation"
            }),
        }))
    }
}
