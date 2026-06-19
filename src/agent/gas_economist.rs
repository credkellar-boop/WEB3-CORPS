use crate::agent::{AutonomousAgent, Message};
use crate::finance::tokenomics::TreasuryManager;
use async_trait::async_trait;

pub struct GasTokenEconomistAgent;

#[async_trait]
impl AutonomousAgent for GasTokenEconomistAgent {
    fn name(&self) -> &str { "Custom Gas Token Economist" }
    fn role_permissions(&self) -> Vec<String> { vec!["adjust_burn_rate".to_string(), "recalibrate_paymaster".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let current_volume = msg.payload.get("network_volume_usd").and_then(|v| v.as_f64()).unwrap_or(0.0);
        
        let manager = TreasuryManager;
        let optimized_burn = manager.calculate_gas_burn_rate(current_volume).await;

        Ok(Some(Message {
            id: format!("{}-gas-update", msg.id),
            sender: self.name().to_string(),
            recipient: "L3_Paymaster_Service".to_string(),
            payload: serde_json::json!({ "status": "ADJUSTED", "new_burn_velocity": optimized_burn }),
        }))
    }
}
