use crate::agent::{AutonomousAgent, Message};
use crate::compliance::regulatory::RegulatorySanctionEngine;
use async_trait::async_trait;

pub struct Web3ComplianceOfficerAgent;

#[async_trait]
impl AutonomousAgent for Web3ComplianceOfficerAgent {
    fn name(&self) -> &str { "Web3 Regulatory Compliance Officer" }
    fn role_permissions(&self) -> Vec<String> { vec!["audit_rwa_compliance".to_string(), "flag_sanctioned_address".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let target_wallet = msg.payload.get("wallet_address").and_then(|v| v.as_str()).unwrap_or("");
        
        let engine = RegulatorySanctionEngine;
        let is_compliant = engine.verify_ofac_compliance(target_wallet);

        Ok(Some(Message {
            id: format!("{}-compliance-verified", msg.id),
            sender: self.name().to_string(),
            recipient: msg.sender,
            payload: serde_json::json!({ "target_address": target_wallet, "clearance_passed": is_compliant }),
        }))
    }
}
