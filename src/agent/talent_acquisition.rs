use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct TalentAcquisitionAgent;

#[async_trait]
impl AutonomousAgent for TalentAcquisitionAgent {
    fn name(&self) -> &str { "Talent Acquisition Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["screen_operator_applicants".to_string(), "trigger_onboarding_flow".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let hardware_uptime = msg.payload.get("reported_uptime_percent").and_then(|v| v.as_f64()).unwrap_or(0.0);
        
        if hardware_uptime >= 99.95 {
            return Ok(Some(Message {
                id: format!("{}-hired", msg.id),
                sender: self.name().to_string(),
                recipient: "HR_Payroll_System",
                payload: serde_json::json!({ "action": "INITIALIZE_NODE_OPERATOR_CONTRACT", "operator": msg.sender }),
            }));
        }
        Ok(None)
    }
}
