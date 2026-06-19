use crate::agent::{AutonomousAgent, Message};
use crate::legal::contract_validator::SmartLegalContractValidator;
use async_trait::async_trait;

pub struct GeneralCounselAgent;

#[async_trait]
impl AutonomousAgent for GeneralCounselAgent {
    fn name(&self) -> &str { "General Counsel Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["approve_legal_settlement".to_string(), "flag_clause_deviation".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let raw_contract_body = msg.payload.get("raw_text").and_then(|v| v.as_str()).unwrap_or("");
        
        let validator = SmartLegalContractValidator;
        let claims_valid = validator.verify_liability_clauses(raw_contract_body);

        let status = if claims_valid { "LEGAL_STANDARDS_MET" } else { "REJECTED_MISSING_PROTECTIVE_CLAUSES" };

        Ok(Some(Message {
            id: format!("{}-legal-review", msg.id),
            sender: self.name().to_string(),
            recipient: msg.sender,
            payload: serde_json::json!({ "status": status, "requires_manual_arbitration": !claims_valid }),
        }))
    }
}
