use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct CeoAgent;

#[async_trait]
impl AutonomousAgent for CeoAgent {
    fn name(&self) -> &str { "CEO / Strategy Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["allocate_capital".to_string(), "authorize_restructure".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let objective = msg.payload.get("macro_objective").and_then(|v| v.as_str()).unwrap_or("");
        
        // Translates high-level growth parameters into targeted deployment targets
        let departmental_target = match objective {
            "expand_rwa" => "CRE_Portfolio_Acquisition_Agent",
            "harden_network" => "Lead_AI_Red_Team_Engineer",
            _ => "Operations_Manager",
        };

        Ok(Some(Message {
            id: format!("{}-directive", msg.id),
            sender: self.name().to_string(),
            recipient: departmental_target.to_string(),
            payload: serde_json::json!({
                "command": "EXECUTE_STRATEGIC_EXPANSION",
                "priority": "CRITICAL"
            }),
        }))
    }
}
