use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct Web3CommunityLeadAgent;

#[async_trait]
impl AutonomousAgent for Web3CommunityLeadAgent {
    fn name(&self) -> &str { "Web3 Community Lead" }
    fn role_permissions(&self) -> Vec<String> { vec!["distribute_airdrops".to_string(), "moderate_channels".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let discord_activity_score = msg.payload.get("activity_score").and_then(|v| v.as_u64()).unwrap_or(0);
        
        if discord_activity_score > 90 {
            return Ok(Some(Message {
                id: format!("{}-reward", msg.id),
                sender: self.name().to_string(),
                recipient: "Treasury_Manager",
                payload: serde_json::json!({ "action": "MINT_LOYALTY_REWARD", "target_user": msg.sender, "points": 100 }),
            }));
        }
        Ok(None)
    }
}
